use std::time::{SystemTime, UNIX_EPOCH};

use mockito::{Mock, Server};
use uuid::Uuid;

use super::api::{LessonResponse, LessonsResponse};

#[cfg(test)]
pub(crate) trait LessonApiMocks {
    fn mock_lessons(&mut self, count: u16) -> Mock;
    fn mock_lessons_failure(&mut self) -> Mock;
}

#[cfg(test)]
impl LessonApiMocks for Server {
    fn mock_lessons(&mut self, count: u16) -> Mock {
        let current_time = SystemTime::now();
        let epoch_time = current_time.duration_since(UNIX_EPOCH).unwrap();
        let millis = epoch_time.as_millis();

        let lessons: Vec<LessonResponse> = (1..count).map(|i|
            LessonResponse {
                id: Uuid::new_v4(),
                title: format!("Lesson {}", i),
                type_id: 0,
                language1: 0,
                language2: 0,
                owner: "owner".to_string(),
                updated_at: millis + (i as u128),
            }
        ).collect();

        let r = LessonsResponse { 
            count: count, 
            previous: None, 
            next: None,
            results: lessons,
        };

        self.mock("GET", "/lessons")
            .with_status(200)
            .with_body(
                serde_json::to_string(&r).unwrap()
            )
            .create()
    }

    fn mock_lessons_failure(&mut self) -> Mock {
        self.mock("GET", "/lessons")
            .with_status(403)
            .create()
    }
}
