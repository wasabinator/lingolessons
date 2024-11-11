use chrono::Utc;
use mockito::{Matcher, Mock, Server};
use uuid::Uuid;

use super::api::{LessonResponse, LessonsResponse};

#[cfg(test)]
pub(crate) trait LessonApiMocks {
    fn mock_lessons_success(&mut self, count: u16, with_deleted: u16, with_session: bool, in_pages: usize) -> Vec<Mock>;
    #[allow(unused)]
    fn mock_lessons_failure(&mut self) -> Mock;
}

#[cfg(test)]
impl LessonApiMocks for Server {
    /// Generates a mock lessons api response.
    /// 
    /// The number of lessons is specified by count
    /// The first 'with_deleted' lessons will be marked as deleted
    fn mock_lessons_success(&mut self, count: u16, with_deleted: u16, with_session: bool, in_pages: usize) -> Vec<Mock> {
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

        let responses: Vec<Vec<LessonResponse>> = lessons.chunks(count as usize / in_pages)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut i = 1;
        responses.iter().map(
            |response| {
                let previous = match i {
                    1 => None,
                    _ => Some(format!("page={}", i))
                };

                let max = lessons.len();
                let next =
                    if i < max {
                        Some(format!("page={}", i + 1))
                    } else {
                        None
                    };

                let r = LessonsResponse {
                    count,
                    previous,
                    next,
                    results: response.clone(),
                };

                let mock = self.mock("GET", "/lessons")
                    .with_status(200)
                    .match_query(Matcher::UrlEncoded("page_no".into(), format!("{}", i)))
                    .with_body(
                        serde_json::to_string(&r).unwrap()
                    );

                let mock = if with_session {
                    mock.match_header("Authorization", "Bearer mock_access_token")
                } else {
                    mock.match_header("Authorization", Matcher::Missing)
                }.create();

                i += 1;

                mock
            }
        ).collect()
    }

    #[allow(unused)] // TODO: This will be used during the future lesson repo sync detail testing
    fn mock_lessons_failure(&mut self) -> Mock {
        self.mock("GET", "/lessons")
            .with_status(403)
            .create()
    }
}
