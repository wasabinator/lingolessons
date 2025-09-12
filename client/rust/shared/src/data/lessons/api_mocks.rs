use super::api::LessonResponse;
use crate::{
    domain::lessons::{Lesson, LessonType},
    DateTime,
};
use chrono::{TimeDelta, Utc};
use mockito::{Mock, Server};
use std::ops::Add;
use uuid::Uuid;

#[cfg(test)]
pub(crate) trait LessonApiMocks {
    fn mock_lessons_success(
        &mut self, lessons: Vec<Lesson>, with_deleted: u16, with_session: bool, in_pages: usize,
        at_timestamp: Option<u64>,
    ) -> Vec<Mock>;

    #[allow(unused)]
    fn mock_lessons_failure(&mut self) -> Mock;
}

// We mock the lessons in domain terms, as the response data type is not available to packages external to data
// Then internally we will map those to the responses for mocking.
pub fn mock_lessons(count: u16) -> Vec<Lesson> {
    let time = DateTime::from(Utc::now());

    (0..count)
        .map(|i| Lesson {
            id: Uuid::new_v4(),
            title: format!("Lesson {i}"),
            r#type: LessonType::Vocabulary,
            language1: "en".to_string(),
            language2: "jp".to_string(),
            owner: "owner".to_string(),
            updated_at: time.add(TimeDelta::seconds(i as i64)),
        })
        .collect()
}

// pub fn mock_lesson(id: Uuid) -> Lesson {
//     let time = DateTime::from(Utc::now());

//     Lesson {
//         id: id,
//         title: format!("Lesson"),
//         r#type: LessonType::Vocabulary,
//         language1: "en".to_string(),
//         language2: "jp".to_string(),
//         owner: "owner".to_string(),
//         updated_at: time,
//     }
// }

fn mock_lesson_responses(lessons: &Vec<Lesson>, with_deleted: u16) -> Vec<LessonResponse> {
    let mut i = 0u16;
    lessons
        .iter()
        .map(|lesson| {
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
        })
        .collect()
}

#[cfg(test)]
impl LessonApiMocks for Server {
    /// Generates a mock lessons api response.
    ///
    /// Lessons is a list of lessons to mock, this list usually is a product of calling mock_lessons()
    /// The first 'with_deleted' lessons will be marked as deleted (useful for testing the sync logic)
    fn mock_lessons_success(
        &mut self, lessons: Vec<Lesson>, with_deleted: u16, with_session: bool, in_pages: usize,
        at_timestamp: Option<u64>,
    ) -> Vec<Mock> {
        use crate::data::api_mocks::mock_api_success;
        use log::debug;
        use std::collections::HashMap;

        debug!("[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[");
        let mock_responses = mock_lesson_responses(&lessons, with_deleted);
        return mock_api_success(
            self,
            "/lessons",
            HashMap::new(),
            lessons,
            mock_responses,
            with_session,
            in_pages,
            at_timestamp,
        );
    }

    #[allow(unused)] // TODO: This will be used during the future lesson repo sync detail testing
    fn mock_lessons_failure(&mut self) -> Mock {
        self.mock("GET", "/lessons").with_status(403).create()
    }
}
