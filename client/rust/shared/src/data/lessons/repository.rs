use std::{borrow::BorrowMut, sync::Arc};
use lru::LruCache;
use uuid::Uuid;
use crate::{common::time::UnixTimestamp,
            data::{api::AuthApi, db::Db, lessons::{api::LessonsApi, db::LessonDao}, SettingRepository},
            domain::{lessons::{Lesson, LessonRepository}, runtime::Runtime, DomainError}, ArcMutex, Run};

use super::{api::LessonResponse, db::LessonData};

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
        runtime: Runtime,
        api: Arc<AuthApi>,
        db: ArcMutex<Db>,
        settings: ArcMutex<SettingRepository>,
        page_cache: LruCache<u8, Vec<Lesson>>,
        lesson_cache: LruCache<Uuid, Lesson>,
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

    pub(in crate::data) fn start(&mut self) {
        log::trace!("lesson_repo::start");

        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        log::trace!("lesson_repo - spawning refresh task");

        self.runtime.spawn(LESSONS_REFRESH_TASK.into(), async move {
            log::debug!("lesson_repo - refresh task started");

            let sync_time = UnixTimestamp::now();

            let mut finished = false;
            let mut page_no: u8 = 0;
            let last_sync_time = settings.launch(
                |settings| async move {
                    settings.get_timestamp(LESSONS_LAST_SYNC_TIME).await
                }
            ).await;

            'sync: while !finished {
                // Try to fetch from the server
                log::trace!("Attempting to fetch lessons from api for page {}", page_no);
                let response = api.get_lessons(
                    page_no,
                    last_sync_time,
                ).await;
                log::trace!("Got response from api {:?}", response);

                match response {
                    Ok(r) => {
                        db.run(
                        |db| {
                            for lesson in r.results {
                                if lesson.is_deleted {
                                    let _ = db.del_lesson(lesson.id);
                                } else {
                                    let data = LessonData::from(lesson);
                                    let _ = db.set_lesson(&data);
                                }
                            }
                        }).await;

                        if r.next.is_none() {
                            // If the next link is null, then we've reached the end.
                            // Set the timestamp so we'll know where to pick up from next sync
                            settings.launch(
                                |settings| async move {
                                    settings.put_timestamp(LESSONS_LAST_SYNC_TIME, sync_time).await;
                                }
                            ).await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to fetch lessons for page {}. Exiting sync: {:?}", page_no, e);
                        break 'sync;
                    }
                };
                log::debug!("lesson_repo - refresh task completed");
            }
        });

        log::trace!("lesson_repo::start finished");
    }

    pub(in crate::data) fn stop(&mut self) {
        log::trace!("lesson_repo::stop");
        let r = self.runtime.borrow_mut();
        r.abort();
    }

    pub(crate) async fn get_lessons(&mut self, page_no: u8) -> anyhow::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to fetch lessons from cache");
        let lessons = match self.page_cache.get(&page_no) {
            Some(lessons) => {
                log::trace!("Got lessons {} from cache", lessons.len());
                lessons.clone()
            },
            None => {
                log::trace!("Cache miss for page {}. Attempting to load lessons from db", page_no);
                let lessons = self.db.run(
                    |db| db.get_lessons()
                ).await?;
                log::trace!("Got lessons {} from db", lessons.len());
                // Map to domain type and cache
                let lessons: Vec<Lesson> = lessons.iter().map(Lesson::from).collect();
                if !lessons.is_empty() {
                    log::trace!("Caching {} lessons for page {}", lessons.len(), page_no);
                    self.page_cache.put(page_no, lessons.clone());
                }
                lessons
            }
        };
        Ok(lessons)
    }

    pub(crate) async fn get_lesson(&mut self, id: Uuid) -> anyhow::Result<Option<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to fetch lesson {} from cache", id);
        let lesson = match self.lesson_cache.get(&id) {
            Some(lesson) => {
                log::trace!("Got lesson from cache");
                Some(lesson.clone())
            },
            None => {
                log::trace!("Cache miss for lesson {}. Attempting to load lesson from db", id);
                let lesson_data = self.db.run(
                    |db| db.get_lesson(id)
                ).await?;
                log::trace!("Got lesson {} from db", id);
                // Map to domain type and cache

                if let Some(lesson) = lesson_data {
                    let lesson = Lesson::from(&lesson);
                    log::trace!("Caching lesson for id {}", id);
                    self.lesson_cache.put(id, lesson.clone());
                    Some(lesson)
                } else {
                    None
                }
            }
        };
        Ok(lesson)
    }
}
