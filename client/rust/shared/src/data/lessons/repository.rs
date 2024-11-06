use std::{borrow::BorrowMut, sync::Arc};

use crate::{data::{api::AuthApi, db::Db, lessons::api::LessonsApi}, domain::{lessons::{Lesson, LessonRepository}, runtime::Runtime, DomainError}, ArcMutex};

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

impl LessonRepository {
    pub(in crate::data) fn new(
        runtime: Runtime,
        api: Arc<AuthApi>,
        db: ArcMutex<Db>,
    ) -> Self {
        LessonRepository {
            runtime,
            api: api.clone(),
            db: db.clone(),
        }
    }

    pub(in crate::data) fn start(&mut self) {
        log::trace!("lesson_repo::start");
        let _r = self.runtime.borrow_mut();

        let api = self.api.clone();
        let db = self.db.clone();
        log::trace!("lesson_repo - spawning refresh task");

        self.runtime.spawn(REFRESH_TASK.into(), async move {
            log::trace!("lesson_repo - refresh task started");
            let _api = api.clone();
            let _db = db.clone();
            log::trace!("lesson_repo - refresh task completed");
        });

        log::trace!("lesson_repo::start finished");
    }

    pub(in crate::data) fn stop(&mut self) {
        log::trace!("lesson_repo::stop");
        let r = self.runtime.borrow_mut();
        r.abort();
    }

    pub(crate) async fn get_lessons(&self) -> uniffi::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        log::trace!("Attempting to load lessons from db");
        let db = self.db.lock().await;
        let lessons = db.get_lessons()?;
        drop(db);

        if !lessons.is_empty() {
            // Lessons have already been cached to the db
            log::trace!("Got lessons from db");
            let lessons = lessons.iter().map(Lesson::from).collect();
            Ok(lessons)
        } else {
            // Try to fetch from the server
            log::trace!("Attempting to fetch lessons from api");
            let response = self.api.get_lessons().await?;
            log::trace!("Got response from api {:?}", response);

            let db = self.db.lock().await;

            let mut lessons = Vec::new();
            for lesson in response.results {
                if lesson.is_deleted {
                    db.del_lesson(lesson.id)?;
                } else {
                    let data = LessonData::from(lesson);
                    db.set_lesson(&data)?;
                    lessons.push(data);
                }
            }

            // Now map to domain type
            let domain = lessons.iter().map(Lesson::from).collect();
            Ok(domain)
        }
    }
}
