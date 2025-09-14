use super::db::{LessonDao, LessonData};
use crate::data::db::Db;
use chrono::Utc;
use uuid::Uuid;

pub(in crate::data) struct DbFixtures {}

impl DbFixtures {
    pub(super) fn create_lessons(db: &Db, count: u16) -> Vec<LessonData> {
        let epoc_time = Utc::now().timestamp();

        let lessons: Vec<LessonData> = (0..count)
            .map(|i| LessonData {
                id: Uuid::new_v4(),
                title: format!("Lesson {i}"),
                r#type: 0,
                language1: "en".to_string(),
                language2: "jp".to_string(),
                owner: "owner".to_string(),
                updated_at: epoc_time + (i as i64),
            })
            .collect();

        for lesson in lessons.clone() {
            db.set_lesson(&lesson).unwrap();
        }

        lessons
    }

    pub(in crate::data) fn insert_lesson(db: &Db, id: Uuid, title: String) {
        let epoc_time = Utc::now().timestamp();
        let data = LessonData {
            id: id,
            title: title,
            r#type: 0,
            language1: "en".to_string(),
            language2: "jp".to_string(),
            owner: "owner".to_string(),
            updated_at: epoc_time,
        };

        let _ = db.set_lesson(&data);
    }
}
