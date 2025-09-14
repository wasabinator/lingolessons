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
};
use log::{debug, error, info, trace};
use std::sync::Arc;
use uuid::Uuid;

impl From<(Uuid, FactResponse)> for FactData {
    fn from(data: (Uuid, FactResponse)) -> Self {
        let (lesson_id, fact) = data;

        FactData {
            id: fact.id,
            lesson_id,
            element1: fact.element1,
            element2: fact.element2,
            hint: fact.hint,
            updated_at: fact.updated_at,
        }
    }
}

impl FactRepository {
    pub(in crate::data) fn new(
        runtime: Runtime,
        api: Arc<AuthApi>,
        db: Arc<Db>,
        settings: Arc<SettingRepository>,
    ) -> Self {
        FactRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
            settings: settings.clone(),
        }
    }

    pub(in crate::data) fn refresh(&self, lesson_id: Uuid) {
        trace!("fact_repo::refresh");

        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        trace!("fact_repo - spawning refresh task");

        let refresh_task_key = format!("FACTS_REFRESH_TASK_{lesson_id}");
        trace!("Refresh task key: {refresh_task_key}");
        self.runtime.spawn(refresh_task_key, async move {
            info!("fact_repo - refresh task started for lesson: {lesson_id}");

            let sync_time = UnixTimestamp::now();
            let last_sync_time_key = format!("FACTS_LAST_SYNC_TIME_{lesson_id}");

            let mut finished = false;
            let mut page_no: u8 = 0;
            let key = last_sync_time_key.clone();
            let last_sync_time = settings.get_timestamp(key.as_str()).await;

            'sync: while !finished {
                // Try to fetch from the server
                info!(
                    "Attempting to fetch facts from api for {}-{}",
                    lesson_id, page_no
                );
                let response = api.get_facts(lesson_id, page_no, last_sync_time).await;
                info!("Got response from api {:?}", response);

                match response {
                    Ok(r) => {
                        for fact in r.results {
                            if fact.is_deleted {
                                let _ = db.del_fact(fact.id);
                            } else {
                                let data = FactData::from((lesson_id, fact));
                                let r = db.set_fact(&data);
                                if r.is_err() {
                                    trace!("could not save fact");
                                } else {
                                    trace!("saved fact");
                                }
                            }
                        }

                        if r.next.is_none() {
                            // If the next link is null, then we've reached the end.
                            // Set the timestamp so we'll know where to pick up from next sync
                            let key = last_sync_time_key.clone();
                            settings.put_timestamp(key.as_str(), sync_time).await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to fetch facts for {lesson_id}-{}. Exiting sync: {:?}",
                            page_no + 1,
                            e
                        );
                        break 'sync;
                    }
                };
                debug!("fact_repo - refresh task completed");
            }
        });

        trace!("fact_repo::start finished");
    }

    pub(crate) fn stop(&self) {
        trace!("lesson_repo::stop");
        self.runtime.abort();
    }

    pub(crate) async fn get_facts(
        &self,
        lesson_id: Uuid,
        page_no: u8,
        page_size: u8,
    ) -> anyhow::Result<Vec<Fact>, DomainError> {
        use super::db::FactDao;

        let facts = self.db.get_facts(lesson_id, page_no, page_size).unwrap();
        trace!("Got facts {} from db", facts.len());
        let facts: Vec<Fact> = facts.iter().map(Fact::from).collect();
        Ok(facts)
    }

    pub(crate) async fn del_facts(&self, lesson_id: Uuid) -> anyhow::Result<(), DomainError> {
        let _ = self.db.del_facts(lesson_id);
        Ok(())
    }
}
