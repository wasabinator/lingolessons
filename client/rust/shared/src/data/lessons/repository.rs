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
use log::{debug, error, trace};
use std::sync::Arc;
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
        runtime: Runtime,
        api: Arc<AuthApi>,
        db: Arc<Db>,
        settings: Arc<SettingRepository>,
    ) -> Self {
        LessonRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
            settings: settings.clone(),
        }
    }

    pub(in crate::data) fn start(&self, fact_repository: Arc<FactRepository>) {
        trace!("lesson_repo::start");
        self.refresh(fact_repository);
    }

    pub(in crate::data) fn refresh(&self, fact_repository: Arc<FactRepository>) {
        let api = self.api.clone();
        let db = self.db.clone();
        let settings = self.settings.clone();
        trace!("lesson_repo - spawning refresh task");

        self.runtime.spawn(LESSONS_REFRESH_TASK.into(), async move {
            debug!("lesson_repo - refresh task started");

            let sync_time = UnixTimestamp::now();

            let mut finished = false;
            let mut page_no: u8 = 0;
            let last_sync_time = settings.get_timestamp(LESSONS_LAST_SYNC_TIME).await;

            'sync: while !finished {
                // Try to fetch from the server
                trace!("Attempting to fetch lessons from api for page {}", page_no);
                let response = api.get_lessons(page_no, last_sync_time).await;
                trace!("Got response from api {:?}", response);

                match response {
                    Ok(r) => {
                        for lesson in r.results {
                            let r = if lesson.is_deleted {
                                let _ = fact_repository.del_facts(lesson.id).await;
                                db.del_lesson(lesson.id)
                            } else {
                                let data = LessonData::from(lesson.clone());
                                let r = db.set_lesson(&data);
                                if r.is_ok() {
                                    debug!("lesson_id: {}, refresh fact repo", lesson.id);
                                    fact_repository.refresh(lesson.id);
                                }
                                r
                            };
                            if r.is_err() {
                                error!("Error refreshing lesson: {r:?}");
                                break 'sync;
                            }
                        }

                        if r.next.is_none() {
                            // If the next link is null, then we've reached the end.
                            // Set the timestamp so we'll know where to pick up from next sync
                            settings
                                .put_timestamp(LESSONS_LAST_SYNC_TIME, sync_time)
                                .await;
                            finished = true;
                        } else {
                            page_no += 1;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to fetch lessons for page {}. Exiting sync: {:?}",
                            page_no, e
                        );
                        break 'sync;
                    }
                };
                debug!("lesson_repo - refresh task completed");
            }
        });

        trace!("lesson_repo::start finished");
    }

    pub(crate) fn stop(&self) {
        trace!("lesson_repo::stop");
        self.runtime.abort();
    }

    pub(crate) async fn get_lessons(
        &self,
        page_no: u8,
        page_size: u8,
    ) -> anyhow::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        let lessons = self.db.get_lessons(page_no, page_size)?;
        trace!("Got lessons {} from db", lessons.len());

        // Map to domain type
        let lessons: Vec<Lesson> = lessons.iter().map(|lesson| Lesson::from(lesson)).collect();

        Ok(lessons)
    }

    pub(crate) async fn get_lesson(&self, id: Uuid) -> anyhow::Result<Option<Lesson>, DomainError> {
        use super::db::LessonDao;

        let lesson_data = self.db.get_lesson(id)?;
        trace!("Got lesson {} from db", id);
        // Map to domain type
        let lesson = if let Some(lesson) = lesson_data {
            let lesson = Lesson::from(&lesson);
            Some(lesson)
        } else {
            None
        };
        Ok(lesson)
    }
}
