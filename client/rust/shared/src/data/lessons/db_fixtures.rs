use chrono::Utc;
use uuid::Uuid;
use crate::data::db::Db;
use super::db::{LessonDao, LessonData};

pub(super) struct DbFixtures {}

impl DbFixtures {
    pub(super) fn create_lessons(db: &Db, count: u16) -> Vec<LessonData> {
        let epoc_time = Utc::now().timestamp();

        let lessons: Vec<LessonData> = (0..count).map(|i|
            LessonData {
                id: Uuid::new_v4(),
                title: format!("Lesson {}", i),
                r#type: 0,
                language1: "en".to_string(),
                language2: "jp".to_string(),
                owner: "owner".to_string(),
                updated_at: epoc_time + (i as i64),
            }
        ).collect();

        for lesson in lessons.clone() {
            let _ = db.set_lesson(&lesson).unwrap();
        }

        lessons
    }
}
