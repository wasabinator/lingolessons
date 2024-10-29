use chrono::{DateTime, Local};
use uniffi::deps::log::trace;

use crate::{data::{api::AuthApi, db::Db}, domain::Domain, ArcMutex};

use super::DomainResult;

/// Lesson domain model
#[derive(uniffi::Record, PartialEq)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub r#type: LessonType,
    pub language1: String,
    pub language2: String,
    pub owner: String,
    pub updated_at: DateTime<Local>,
}

/// Lesson type domain model
#[derive(uniffi::Enum, PartialEq)]
pub enum LessonType {
    Vocabulary,
    Grammar,
}

/// Repository the domain requires for getting and updating lessons
pub(crate) struct LessonRepository {
    pub(crate) api: ArcMutex<AuthApi>,
    pub(crate) db: ArcMutex<Db>,
}

pub trait Lessons {
    fn get_lessons(&self) -> impl std::future::Future<Output = DomainResult<Vec<Lesson>>> + Send;
    fn get_lesson(&self, id: String) -> impl std::future::Future<Output = DomainResult<Option<Lesson>>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Lessons for Domain {
    async fn get_lessons(&self) -> DomainResult<Vec<Lesson>> {
        trace!("get_lessons");
        let repository = self.provider.lesson_repository.lock().await;
        let lessons = repository.get_lessons().await?;
        Ok(lessons)
    }

    async fn get_lesson(&self, _id: String) -> DomainResult<Option<Lesson>> {
        trace!("get_lesson");
        Ok(None) // TODO: Implement when client has a need to fetch lesson details
    }
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use crate::{data::lessons::api_mocks::LessonApiMocks, domain::fake_domain};

    use super::*;

    #[tokio::test]
    async fn test_get_lessons_success() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").unwrap();

        server.deref_mut().mock_lessons_success(5, 0);

        let r = domain.get_lessons().await;
        assert!(r.is_ok());
        assert_eq!(5, r.unwrap().len());
    }

    #[tokio::test]
    async fn test_get_lessons_doesnt_persist_deleted_items() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").unwrap();

        server.deref_mut().mock_lessons_success(5, 1);

        let r = domain.get_lessons().await;
        assert!(r.is_ok());
        assert_eq!(4, r.unwrap().len());
    }

    #[tokio::test]
    async fn test_get_lessons_failure() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").unwrap();

        server.deref_mut().mock_lessons_failure();

        let r = domain.get_lessons().await;
        assert!(r.is_err());
    }
}
