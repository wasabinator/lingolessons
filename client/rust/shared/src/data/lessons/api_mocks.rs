use super::api::{LessonResponse, LessonsResponse};
use crate::domain::lessons::{Lesson, LessonType};
use crate::DateTime;
use chrono::{TimeDelta, Utc};
use mockito::{Matcher, Mock, Server};
use std::ops::Add;
use uuid::Uuid;

#[cfg(test)]
pub(crate) trait LessonApiMocks {
    fn mock_lessons_success(
        &mut self,
        lessons: Vec<Lesson>,
        with_deleted: u16,
        with_session: bool,
        in_pages: usize,
        at_timestamp: Option<u64>
    ) -> Vec<Mock>;

    #[allow(unused)]
    fn mock_lessons_failure(&mut self) -> Mock;
}

// We mock the lessons in domain terms, as the response data type is not available to packages external to data
// Then internally we will map those to the responses for mocking.
pub fn mock_lessons(
    count: u16,
) -> Vec<Lesson> {
    let time = DateTime::from(Utc::now());

    (0..count).map(|i|
        Lesson {
            id: Uuid::new_v4(),
            title: format!("Lesson {}", i),
            r#type: LessonType::Vocabulary,
            language1: "en".to_string(),
            language2: "jp".to_string(),
            owner: "owner".to_string(),
            updated_at: time.add(TimeDelta::seconds(i as i64)),
        }
    ).collect()
}

fn mock_lesson_responses(
    lessons: Vec<Lesson>,
    with_deleted: u16,
) -> Vec<LessonResponse> {
    let mut i = 0u16;
    lessons.iter().map(|lesson| {
        let response = LessonResponse {
            id: lesson.id,
            title: lesson.title.clone(),
            r#type: 0,
            language1: lesson.language1.clone(),
            language2: lesson.language2.clone(),
            owner: lesson.owner.clone(),
            is_deleted: i < with_deleted,
            updated_at: lesson.updated_at.to_utc().timestamp(),
        };
        i += 1;
        response
    }).collect()
}

#[cfg(test)]
impl LessonApiMocks for Server {
    /// Generates a mock lessons api response.
    ///
    /// Lessons is a list of lessons to mock, this list usually is a product of calling mock_lessons()
    /// The first 'with_deleted' lessons will be marked as deleted (useful for testing the sync logic)
    fn mock_lessons_success(
        &mut self,
        lessons: Vec<Lesson>,
        with_deleted: u16,
        with_session: bool,
        in_pages: usize,
        at_timestamp: Option<u64>,
    ) -> Vec<Mock> {
        let count = lessons.len();
        let lessons = mock_lesson_responses(
            lessons,
            with_deleted,
        );
        let responses: Vec<Vec<LessonResponse>> = lessons.chunks(count / in_pages)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut i = 1;
        let max = responses.len();

        responses.iter().map(
            |response| {
                let previous = match i {
                    1 => None,
                    _ => Some(format!("page={}", i))
                };

                let next =
                    if i < max {
                        Some(format!("page={}", i + 1))
                    } else {
                        None
                    };

                let r = LessonsResponse {
                    count: count as u16,
                    previous,
                    next,
                    results: response.clone(),
                };

                let mut mock = self.mock("GET", "/lessons")
                    .with_status(200)
                    .match_query(Matcher::UrlEncoded("page_no".into(), format!("{}", i)))
                    .with_body(
                        serde_json::to_string(&r).unwrap()
                    );

                mock = if with_session {
                    mock.match_header("Authorization", "Bearer mock_access_token")
                } else {
                    mock.match_header("Authorization", Matcher::Missing)
                };

                mock = if let Some(timestamp) = at_timestamp {
                    mock.match_query(Matcher::UrlEncoded("since".into(), timestamp.to_string()))
                } else {
                    mock
                };

                i += 1;

                mock.create()
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
