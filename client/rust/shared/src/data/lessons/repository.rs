use std::{borrow::BorrowMut, sync::Arc};

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

static REFRESH_TASK: &str = "REFRESH_TASK";
static LESSONS_LAST_SYNC_TIME: &str = "LESSONS_LAST_SYNC_TIME";
#[allow(unused)] // Implementing shortly
const PAGE_SIZE: u8 = 2;

impl LessonRepository {
    pub(in crate::data) fn new(
        runtime: Runtime,
        api: Arc<AuthApi>,
        db: ArcMutex<Db>,
        settings: ArcMutex<SettingRepository>
    ) -> Self {
        LessonRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
            settings: settings.clone(),
        }
    }

    pub(in crate::data) fn start(&mut self) {
        log::trace!("lesson_repo::start");

        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        log::trace!("lesson_repo - spawning refresh task");

        self.runtime.spawn(REFRESH_TASK.into(), async move {
            log::trace!("lesson_repo - refresh task started");
            log::trace!("lesson_repo - refresh task completed");

            let mut finished = false;
            #[allow(unused)] // TODO: Implementing shortly
            let mut page_no: u8 = 0;
            #[allow(unused)] // TODO: Implementing shortly
            let timestamp = settings.launch(
                |settings| async move {
                    settings.get_timestamp(LESSONS_LAST_SYNC_TIME).await;
                }
            ).await;

            'sync: while !finished {
                // Try to fetch from the server
                log::trace!("Attempting to fetch lessons from api");
                let response = api.get_lessons().await;
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
                                    settings.put_timestamp(LESSONS_LAST_SYNC_TIME, UnixTimestamp::now()).await;
                                }
                            ).await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to fetch lessons: {:?}", e);
                        break 'sync;
                    }
                };
            }
        });

        log::trace!("lesson_repo::start finished");
    }

    pub(in crate::data) fn stop(&mut self) {
        log::trace!("lesson_repo::stop");
        let r = self.runtime.borrow_mut();
        r.abort();
    }

    pub(crate) async fn get_lessons(&self) -> anyhow::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to load lessons from db");
        let lessons = self.db.run(
            |db| {
                db.get_lessons()
            }
        ).await?;

        Ok(
            lessons.iter().map(Lesson::from).collect()
        )
    }
}
