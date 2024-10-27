use crate::{data::{api::Api, db::Db, lessons::api::LessonsApi}, domain::{lessons::{Lesson, LessonRepository}, DomainError}, ArcMutex};

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

impl LessonRepository {
    pub(in crate::data) fn new(api: ArcMutex<Api>, db: ArcMutex<Db>) -> Self {
        LessonRepository {
            api: api.clone(),
            db: db.clone()
        }
    }

    pub(crate) async fn get_lessons(&self) -> uniffi::Result<Vec<Lesson>, DomainError> {
        use super::db::LessonDao;

        let db = self.db.lock().await;
        let lessons = db.get_lessons()?;

        if !lessons.is_empty() {
            // Lessons have already been cached to the db
            let lessons = lessons.iter().map(
                Lesson::from
            ).collect();
            Ok(lessons)
        } else {
            // Try to fetch from the server
            let api = self.api.lock().await;
            let response = api.get_lessons().await?;
            let lessons: Vec<LessonData> = response.results.into_iter().map(LessonData::from).collect();

            // Save to the db
            for lesson in &lessons {
                db.set_lesson(
                    lesson
                )?;
            }

            // Now map to domain type
            let domain = lessons.iter().map(Lesson::from).collect();
            Ok(domain)
        }
    }
}
