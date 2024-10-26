use chrono::Utc;
use mockito::{Mock, Server};
use uuid::Uuid;

use super::api::{LessonResponse, LessonsResponse};

#[cfg(test)]
pub(crate) trait ApiMocks {
    fn mock_lessons(&mut self, count: u16) -> Mock;
    fn mock_lessons_failure(&mut self) -> Mock;
}

#[cfg(test)]
impl ApiMocks for Server {
    fn mock_lessons(&mut self, count: u16) -> Mock {
        let epoc_time = Utc::now().timestamp();

        let lessons: Vec<LessonResponse> = (1..count).map(|i|
            LessonResponse {
                id: Uuid::new_v4(),
                title: format!("Lesson {}", i),
                type_id: 0,
                language1: 0,
                language2: 0,
                owner: "owner".to_string(),
                updated_at: epoc_time + (i as i64),
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
