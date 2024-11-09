use std::sync::Arc;

use chrono::{DateTime, Local};
use lru::LruCache;
use uniffi::deps::log::trace;

use crate::{data::{api::AuthApi, db::Db}, domain::Domain, ArcMutex};

use super::{runtime::Runtime, settings::SettingRepository, DomainResult};

/// Lesson domain model
#[derive(uniffi::Record, Clone, PartialEq)]
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
#[derive(uniffi::Enum, Clone, PartialEq)]
pub enum LessonType {
    Vocabulary,
    Grammar,
}

/// Repository the domain requires for getting and updating lessons
pub(crate) struct LessonRepository {
    pub(crate) runtime: Runtime,
    pub(crate) api: Arc<AuthApi>,
    pub(crate) db: ArcMutex<Db>,
    pub(crate) settings: ArcMutex<SettingRepository>,
    pub(crate) cache: LruCache<u8, Vec<Lesson>>,
}

pub trait Lessons {
    fn get_lessons(&self, page_no: u8) -> impl std::future::Future<Output = DomainResult<Vec<Lesson>>> + Send;
    fn get_lesson(&self, id: String) -> impl std::future::Future<Output = DomainResult<Option<Lesson>>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Lessons for Domain {
    async fn get_lessons(&self, page_no: u8) -> DomainResult<Vec<Lesson>> {
        trace!("get_lessons");

        let lesson_repository = self.provider.lesson_repository.clone();
        let mut repo = lesson_repository.lock().await;

        trace!("About to call repository::get_lessons()");
        let lessons = repo.get_lessons(page_no).await?;

        trace!("Received lessons");
        Ok(lessons)
    }

    async fn get_lesson(&self, _id: String) -> DomainResult<Option<Lesson>> {
        trace!("get_lesson");
        Ok(None) // TODO: Implement when client has a need to fetch lesson details
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test::await_condition;
    use crate::{data::{auth::api_mocks::TokenApiMocks, lessons::api_mocks::LessonApiMocks},
                domain::{auth::Auth, fake_domain}};
    use std::ops::DerefMut;

    #[tokio::test]
    async fn test_get_lessons_with_session_success() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_lessons_success(5, 0, true);
        server.deref_mut().mock_login_success();

        let _ = domain.login("user".to_string(), "password".to_string()).await;

        // We wrap this check around a timeout
        let r = await_condition(
            || async { domain.get_lessons(0).await.unwrap().len() },
            |count| *count == 5,
        ).await;

        assert!(r.is_ok());
        assert_eq!(5, r.unwrap());
    }

    #[tokio::test]
    async fn test_get_lessons_doesnt_persist_deleted_items() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_login_success();
        server.deref_mut().mock_lessons_success(5, 1, true);

        let _ = domain.login("user".to_string(), "password".to_string()).await;

        let r = await_condition(
            || async { domain.get_lessons(0).await.unwrap().len() },
            |count| *count == 4,
        ).await;

        assert!(r.is_ok());
        assert_eq!(4, r.unwrap());
    }
}
