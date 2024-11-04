use chrono::Utc;
use mockito::{Matcher, Mock, Server};
use uuid::Uuid;

use super::api::{LessonResponse, LessonsResponse};

#[cfg(test)]
pub(crate) trait LessonApiMocks {
    fn mock_lessons_success(&mut self, count: u16, with_deleted: u16, with_session: bool) -> Mock;
    fn mock_lessons_failure(&mut self) -> Mock;
}

#[cfg(test)]
impl LessonApiMocks for Server {
    /// Generates a mock lessons api response.
    /// 
    /// The number of lessons is specified by count
    /// The first 'with_deleted' lessons will be marked as deleted
    fn mock_lessons_success(&mut self, count: u16, with_deleted: u16, with_session: bool) -> Mock {
        let epoc_time = Utc::now().timestamp();

        let lessons: Vec<LessonResponse> = (0..count).map(|i|
            LessonResponse {
                id: Uuid::new_v4(),
                title: format!("Lesson {}", i),
                r#type: 0,
                language1: "en".to_string(),
                language2: "jp".to_string(),
                owner: "owner".to_string(),
                is_deleted: i < with_deleted,
                updated_at: epoc_time + (i as i64),
            }
        ).collect();

        let r = LessonsResponse { 
            count: count, 
            previous: None, 
            next: None,
            results: lessons,
        };

        let mock = self.mock("GET", "/lessons")
            .with_status(200)
            .with_body(
                serde_json::to_string(&r).unwrap()
            );
        
        if with_session {
            mock.match_header("Authorization", "Bearer mock_access_token")
        } else { 
            mock.match_header("Authorization", Matcher::Missing)
        }.create()
    }

    fn mock_lessons_failure(&mut self) -> Mock {
        self.mock("GET", "/lessons")
            .with_status(403)
            .create()
    }
}
