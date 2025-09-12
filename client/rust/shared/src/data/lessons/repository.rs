use super::{api::LessonResponse, db::LessonData};
use crate::{
    common::time::UnixTimestamp,
    data::{
        api::AuthApi,
        db::Db,
        lessons::{api::LessonsApi, db::LessonDao},
        SettingRepository,
    },
    domain::{
        facts::FactRepository,
        lessons::{Lesson, LessonRepository},
        runtime::Runtime,
        DomainError,
    },
};
use log::debug;
use lru::LruCache;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

impl From<LessonResponse> for LessonData {
    fn from(lesson: LessonResponse) -> Self {
        LessonData {
            id: lesson.id,
            title: lesson.title,
            r#type: lesson.r#type,
            language1: lesson.language1,
            language2: lesson.language2,
            owner: lesson.owner,
            updated_at: lesson.updated_at,
        }
    }
}

static LESSONS_REFRESH_TASK: &str = "LESSONS_REFRESH_TASK";
static LESSONS_LAST_SYNC_TIME: &str = "LESSONS_LAST_SYNC_TIME";

#[allow(unused)] // Implementing shortly
const PAGE_SIZE: u8 = 20;

impl LessonRepository {
    pub(in crate::data) fn new(
        runtime: Runtime, api: Arc<AuthApi>, db: Arc<Db>, settings: Arc<SettingRepository>,
        page_cache: RwLock<LruCache<u8, Vec<Lesson>>>,
        lesson_cache: RwLock<LruCache<Uuid, Lesson>>,
    ) -> Self {
        LessonRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
            settings: settings.clone(),
            page_cache,
            lesson_cache,
        }
    }

    pub(in crate::data) fn start(&self, fact_repository: Arc<FactRepository>) {
        log::trace!("lesson_repo::start");
        self.refresh(fact_repository);
    }

    pub(in crate::data) fn refresh(&self, fact_repository: Arc<FactRepository>) {
        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        log::trace!("lesson_repo - spawning refresh task");

        self.runtime.spawn(LESSONS_REFRESH_TASK.into(), async move {
            log::debug!("lesson_repo - refresh task started");

            let sync_time = UnixTimestamp::now();

            let mut finished = false;
            let mut page_no: u8 = 0;
            let last_sync_time = settings.get_timestamp(LESSONS_LAST_SYNC_TIME).await;

            'sync: while !finished {
                // Try to fetch from the server
                log::trace!("Attempting to fetch lessons from api for page {}", page_no);
                let response = api.get_lessons(page_no, last_sync_time).await;
                log::trace!("Got response from api {:?}", response);

                match response {
                    Ok(r) => {
                        let res = r.results.clone();

                        for lesson in r.results {
                            let r = if lesson.is_deleted {
                                db.del_lesson(lesson.id)
                            } else {
                                let data = LessonData::from(lesson);
                                db.set_lesson(&data)
                            };
                            if r.is_err() {
                                break;
                            }
                        }

                        for lesson in res {
                            if lesson.is_deleted {
                                if (fact_repository.del_facts(lesson.id)).await.is_err() {
                                    break;
                                }
                            } else {
                                debug!(">>>>>>>>>>> lesson_id: {}, refresh fact repo", lesson.id);
                                fact_repository.refresh(lesson.id);
                            }
                        }

                        if r.next.is_none() {
                            // If the next link is null, then we've reached the end.
                            // Set the timestamp so we'll know where to pick up from next sync
                            settings.put_timestamp(LESSONS_LAST_SYNC_TIME, sync_time).await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to fetch lessons for page {}. Exiting sync: {:?}",
                            page_no,
                            e
                        );
                        break 'sync;
                    }
                };
                log::debug!("lesson_repo - refresh task completed");
            }
        });

        log::trace!("lesson_repo::start finished");
    }

    pub(crate) fn stop(&self) {
        log::trace!("lesson_repo::stop");
        self.runtime.abort();
    }

    pub(crate) async fn get_lessons(
        &self, page_no: u8,
    ) -> anyhow::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to fetch lessons from cache");
        let mut cache = self.page_cache.write().unwrap();
        let lessons = match cache.get(&page_no) {
            Some(lessons) => {
                log::trace!("Got lessons {} from cache", lessons.len());
                lessons.clone()
            }
            None => {
                log::trace!("Cache miss for page {}. Attempting to load lessons from db", page_no);
                let lessons = self.db.get_lessons()?;
                log::trace!("Got lessons {} from db", lessons.len());
                // Map to domain type and cache
                let lessons: Vec<Lesson> = lessons.iter().map(Lesson::from).collect();
                if !lessons.is_empty() {
                    log::trace!("Caching {} lessons for page {}", lessons.len(), page_no);
                    cache.put(page_no, lessons.clone());
                }
                lessons
            }
        };
        Ok(lessons)
    }

    pub(crate) async fn get_lesson(&self, id: Uuid) -> anyhow::Result<Option<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to fetch lesson {} from cache", id);
        let mut cache = self.lesson_cache.write().unwrap();
        let lesson = match cache.get(&id) {
            Some(lesson) => {
                log::trace!("Got lesson from cache");
                Some(lesson.clone())
            }
            None => {
                log::trace!("Cache miss for lesson {}. Attempting to load lesson from db", id);
                let lesson_data = self.db.get_lesson(id)?;
                log::trace!("Got lesson {} from db", id);
                // Map to domain type and cache

                if let Some(lesson) = lesson_data {
                    let lesson = Lesson::from(&lesson);
                    log::trace!("Caching lesson for id {}", id);
                    cache.put(id, lesson.clone());
                    Some(lesson)
                } else {
                    None
                }
            }
        };
        Ok(lesson)
    }
}
