use super::{api::FactResponse, db::FactData};
use crate::{
    common::time::UnixTimestamp,
    data::{
        api::AuthApi,
        db::Db,
        facts::{api::FactsApi, db::FactDao},
        SettingRepository,
    },
    domain::{
        facts::{Fact, FactRepository},
        runtime::Runtime,
        DomainError,
    },
    ArcMutex, Run,
};
use lru::LruCache;
use std::{borrow::BorrowMut, sync::Arc};
use uuid::Uuid;

impl From<(Uuid, FactResponse)> for FactData {
    fn from(data: (Uuid, FactResponse)) -> Self {
        let (lesson_id, fact) = data;

        FactData {
            id: fact.id,
            lesson_id: lesson_id,
            element1: fact.element1,
            element2: fact.element2,
            hint: fact.hint,
            updated_at: fact.updated_at,
        }
    }
}

#[allow(unused)] // Implementing shortly
const PAGE_SIZE: u8 = 20;

impl FactRepository {
    pub(in crate::data) fn new(
        runtime: Runtime, api: Arc<AuthApi>, db: ArcMutex<Db>,
        settings: ArcMutex<SettingRepository>, page_cache: LruCache<(Uuid, u8), Vec<Fact>>,
    ) -> Self {
        FactRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
            settings: settings.clone(),
            page_cache: page_cache,
        }
    }

    pub(in crate::data) fn refresh(&mut self, lesson_id: Uuid) {
        log::trace!("fact_repo::start");

        let lesson_id = lesson_id.clone();
        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        log::trace!("fact_repo - spawning refresh task");

        let refresh_task_key = format!("FACTS_REFRESH_TASK_{lesson_id}");

        self.runtime.spawn(refresh_task_key, async move {
            log::info!("fact_repo - refresh task started for lesson: {lesson_id}");

            let sync_time = UnixTimestamp::now();
            let last_sync_time_key = format!("FACTS_LAST_SYNC_TIME_{lesson_id}");

            let mut finished = false;
            let mut page_no: u8 = 0;
            let key = last_sync_time_key.clone();
            let last_sync_time = settings
                .launch(|settings| async move { settings.get_timestamp(key.as_str()).await })
                .await;

            'sync: while !finished {
                // Try to fetch from the server
                log::info!("Attempting to fetch facts from api for page {}", page_no);
                let response = api.get_facts(lesson_id, page_no, last_sync_time).await;
                log::info!("Got response from api {:?}", response);

                match response {
                    Ok(r) => {
                        db.run(|db| {
                            for fact in r.results {
                                if fact.is_deleted {
                                    let _ = db.del_fact(fact.id);
                                } else {
                                    let data = FactData::from((lesson_id, fact));
                                    let _ = db.set_fact(&data);
                                }
                            }
                        })
                        .await;

                        if r.next.is_none() {
                            // If the next link is null, then we've reached the end.
                            // Set the timestamp so we'll know where to pick up from next sync
                            let key = last_sync_time_key.clone();
                            settings
                                .launch(|settings| async move {
                                    settings.put_timestamp(key.as_str(), sync_time).await;
                                })
                                .await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to fetch facts for page {}. Exiting sync: {:?}",
                            page_no + 1,
                            e
                        );
                        break 'sync;
                    }
                };
                log::debug!("fact_repo - refresh task completed");
            }
        });

        log::trace!("fact_repo::start finished");
    }

    pub(crate) fn stop(&mut self) {
        log::trace!("lesson_repo::stop");
        let r = self.runtime.borrow_mut();
        r.abort();
    }

    pub(crate) async fn get_facts(
        &mut self, lesson_id: Uuid, page_no: u8,
    ) -> anyhow::Result<Vec<Fact>, DomainError> {
        use super::db::FactDao;

        log::trace!("Attempting to fetch facts from cache");
        let facts = match self.page_cache.get(&(lesson_id, page_no)) {
            Some(facts) => {
                log::trace!("Got facts {} from cache", facts.len());
                facts.clone()
            }
            None => {
                log::trace!("Cache miss for page {}. Attempting to load facts from db", page_no);
                let facts = self.db.run(|db| db.get_facts(lesson_id)).await?;
                log::trace!("Got facts {} from db", facts.len());
                // Map to domain type and cache
                let facts: Vec<Fact> = facts.iter().map(Fact::from).collect();
                if !facts.is_empty() {
                    log::trace!("Caching {} facts for page {}{}", facts.len(), lesson_id, page_no);
                    self.page_cache.put((lesson_id, page_no), facts.clone());
                }
                facts
            }
        };
        Ok(facts)
    }

    pub(crate) async fn del_facts(&mut self, lesson_id: Uuid) -> anyhow::Result<(), DomainError> {
        let _ = self.db.run(|db| db.del_facts(lesson_id)).await;
        Ok(())
    }
}
