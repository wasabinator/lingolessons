use chrono::Utc;
use uuid::Uuid;

use super::db::Lesson;

pub(super) struct DbFixtures {
}

impl DbFixtures {
    pub fn create_lessons(count: u16) -> Vec<Lesson> {
        let epoc_time = Utc::now().timestamp();

        (0..count).map(|i|
            Lesson {
                id: Uuid::new_v4(),
                title: format!("Lesson {}", i),
                r#type: 0,
                language1: "en".to_string(),
                language2: "jp".to_string(),
                owner: "owner".to_string(),
                updated_at: epoc_time + (i as i64),
            }
        ).collect()
    }
}
