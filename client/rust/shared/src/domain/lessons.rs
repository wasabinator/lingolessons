use super::{runtime::Runtime, settings::SettingRepository, DomainResult};
use crate::{
    data::{api::AuthApi, db::Db},
    domain::Domain,
};
use chrono::{DateTime, Local};
use std::sync::Arc;
use uniffi::deps::log::trace;
use uuid::Uuid;

/// Lesson domain model
#[derive(uniffi::Record, Clone, PartialEq)]
pub struct Lesson {
    pub id: Uuid,
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
    pub(crate) db: Arc<Db>,
    pub(crate) settings: Arc<SettingRepository>,
}

pub trait Lessons {
    fn get_lessons(
        &self,
        page_no: u8,
        page_size: u8,
    ) -> impl std::future::Future<Output = DomainResult<Vec<Lesson>>> + Send;
    fn get_lesson(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = DomainResult<Option<Lesson>>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Lessons for Domain {
    async fn get_lessons(&self, page_no: u8, page_size: u8) -> DomainResult<Vec<Lesson>> {
        trace!("get_lessons");

        let repo = self.provider.lesson_repository.clone();

        trace!("About to call repository::get_lessons()");
        let lessons = repo.get_lessons(page_no, page_size).await?;

        trace!("Received lessons");
        Ok(lessons)
    }

    async fn get_lesson(&self, id: Uuid) -> DomainResult<Option<Lesson>> {
        trace!("get_lesson");

        let repo = self.provider.lesson_repository.clone();

        trace!("About to call repository::get_lessons()");
        let lesson = repo.get_lesson(id).await?;

        trace!("Received optional lesson from repo");
        Ok(lesson)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::test::{await_condition, await_condition_arg},
        data::{
            auth::api_mocks::TokenApiMocks,
            facts::api_mocks::FactsApiMocks,
            lessons::api_mocks::{mock_lessons, LessonApiMocks},
        },
        domain::{auth::Auth, facts::Facts, fake_domain},
    };
    use serial_test::serial;
    use std::ops::DerefMut;

    #[serial]
    #[tokio::test]
    async fn test_get_lessons_with_session_success() {
        let mut server = mockito::Server::new_async().await;
        let lessons = mock_lessons(5);
        for lesson in &lessons {
            server
                .deref_mut()
                .mock_facts_success(lesson.id, Vec::new(), 0, true, 1, None);
        }
        server
            .deref_mut()
            .mock_lessons_success(lessons, 0, true, 1, None);
        server.deref_mut().mock_login_success();

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        // We wrap this check around a timeout
        let r = await_condition(
            || async { domain.get_lessons(0, 10).await.unwrap().len() },
            |count| *count == 5,
        )
        .await;

        let _ = domain.stop();

        assert!(r.is_ok());
        assert_eq!(5, r.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_lesson_with_session_success() {
        let mut server = mockito::Server::new_async().await;
        // Mock a response and grab the first id
        let lessons = mock_lessons(1);
        let expected_id = lessons.first().unwrap().id;

        for lesson in &lessons {
            server
                .deref_mut()
                .mock_facts_success(lesson.id, Vec::new(), 0, true, 1, None);
        }
        server
            .deref_mut()
            .mock_lessons_success(lessons, 0, true, 1, None);
        server.deref_mut().mock_login_success();

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        // We wrap this check around a timeout
        let r = await_condition_arg(
            || async {
                domain
                    .get_lesson(expected_id)
                    .await
                    .unwrap()
                    .map_or_else(Uuid::new_v4, |lesson| lesson.id)
            },
            &expected_id,
            |id, id2| *id == *id2,
        )
        .await;

        let _ = domain.stop();

        assert!(r.is_ok());
        assert_eq!(expected_id, r.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_lessons_doesnt_persist_deleted_items() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let lessons = mock_lessons(5);
        for lesson in &lessons {
            server
                .deref_mut()
                .mock_facts_success(lesson.id, Vec::new(), 0, true, 1, None);
        }
        server
            .deref_mut()
            .mock_lessons_success(lessons, 1, true, 1, None);

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let r = await_condition(
            || async { domain.get_lessons(0, 10).await.unwrap().len() },
            |count| *count == 4,
        )
        .await;

        let _ = domain.stop();

        assert!(r.is_ok());
        assert_eq!(4, r.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_lesson_sync_saves_timestamp_on_success() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let lessons = mock_lessons(10);
        for lesson in &lessons {
            server
                .deref_mut()
                .mock_facts_success(lesson.id, Vec::new(), 0, true, 1, None);
        }
        server
            .deref_mut()
            .mock_lessons_success(lessons, 0, true, 1, None);

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let settings = domain.provider.setting_repository.clone();
        let r = await_condition(
            || async { settings.get_timestamp("LESSONS_LAST_SYNC_TIME").await },
            |timestamp| timestamp.is_some(),
        )
        .await;

        let _ = domain.stop();

        assert!(r.is_ok());
        assert!(r.unwrap().is_some()) // A timestamp should now exist
    }

    #[serial]
    #[tokio::test]
    async fn test_lesson_sync_uses_saved_timestamp() {
        let mut server = mockito::Server::new_async().await;
        let lessons = mock_lessons(2);
        server.deref_mut().mock_login_success();
        for lesson in &lessons {
            server
                .deref_mut()
                .mock_facts_success(lesson.id, vec![], 0, true, 1, None);
        }
        server
            .deref_mut()
            .mock_lessons_success(lessons, 0, true, 1, Some(1337));

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let settings = domain.provider.setting_repository.clone();

        // Insert a mock timestamp
        settings.put_timestamp("LESSONS_LAST_SYNC_TIME", 1337).await;

        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let r = await_condition(
            || async {
                let r = domain.get_lessons(0, 10).await;
                r.unwrap().len()
            },
            |count| *count == 2,
        )
        .await;

        let _ = domain.stop();

        assert!(r.is_ok());
        assert_eq!(2, r.unwrap()); // Expect only the "updated" lessons
    }
}
