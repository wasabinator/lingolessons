use crate::{
    data::{api::AuthApi, db::Db},
    domain::{runtime::Runtime, settings::SettingRepository, Domain, DomainResult},
};
use log::trace;
use lru::LruCache;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Fact domain model
#[derive(uniffi::Record, Clone, PartialEq)]
pub struct Fact {
    pub id: Uuid,
    pub lesson_id: Uuid,
    pub element1: String,
    pub element2: String,
    pub hint: String,
}

/// Repository the domain requires for getting facts
pub(crate) struct FactRepository {
    pub(crate) runtime: Runtime,
    pub(crate) api: Arc<AuthApi>,
    pub(crate) db: Arc<Db>,
    pub(crate) settings: Arc<SettingRepository>,
    pub(crate) page_cache: RwLock<LruCache<(Uuid, u8), Vec<Fact>>>,
}

pub trait Facts {
    fn get_facts(
        &self,
        lesson_id: Uuid,
        page_no: u8,
    ) -> impl std::future::Future<Output = DomainResult<Vec<Fact>>> + Sync;

    fn stop(&self) -> impl std::future::Future<Output = ()>;
}

#[uniffi::export(async_runtime = "tokio")]
impl Facts for Domain {
    async fn get_facts(&self, lesson_id: Uuid, page_no: u8) -> DomainResult<Vec<Fact>> {
        trace!("get_facts");
        let facts = self
            .provider
            .fact_repository
            .get_facts(lesson_id, page_no)
            .await?;
        trace!("Received facts");
        Ok(facts)
    }

    async fn stop(&self) {
        self.provider.fact_repository.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::test::await_condition,
        data::{
            auth::api_mocks::TokenApiMocks,
            facts::api_mocks::{mock_facts, FactsApiMocks},
            lessons::api_mocks::{mock_lessons, LessonApiMocks},
        },
        domain::{auth::Auth, fake_domain},
    };
    use serial_test::serial;
    use std::ops::DerefMut;

    #[serial]
    #[tokio::test]
    async fn test_get_facts_with_session_success() {
        let mut server = mockito::Server::new_async().await;
        let lessons = mock_lessons(5);
        server
            .deref_mut()
            .mock_lessons_success(lessons.clone(), 0, true, 1, None);
        for i in 0..=4 {
            let id = lessons[i].id;
            server
                .deref_mut()
                .mock_facts_success(id, mock_facts(id, i + 1), 0, true, 1, None);
        }
        server.deref_mut().mock_login_success();

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let r = await_condition(
            || async {
                let r = domain.get_facts(lessons[0].id, 0).await.unwrap().len();
                log::debug!("facts: {r}");
                r
            },
            |count| *count == 1,
        )
        .await;

        assert!(r.is_ok());
        assert_eq!(1, r.unwrap());

        let r = await_condition(
            || async { domain.get_facts(lessons[4].id, 0).await.unwrap().len() },
            |count| *count == 5,
        )
        .await;

        assert!(r.is_ok());
        assert_eq!(5, r.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_facts_doesnt_persist_deleted_items() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let lessons = mock_lessons(1);
        server
            .deref_mut()
            .mock_lessons_success(lessons.clone(), 0, true, 1, None);
        let lesson_id = lessons[0].id;
        server.deref_mut().mock_facts_success(
            lesson_id,
            mock_facts(lesson_id, 5),
            2,
            true,
            1,
            None,
        );

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let r = await_condition(
            || async {
                let r = domain.get_facts(lesson_id, 0).await.unwrap().len();
                log::debug!("facts: {r}");
                r
            },
            |count| *count == 3, // 5 facts, 2 deleted
        )
        .await;

        assert!(r.is_ok());
        assert_eq!(3, r.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_fact_sync_saves_timestamp_on_success() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let lessons = mock_lessons(1);
        server
            .deref_mut()
            .mock_lessons_success(lessons.clone(), 0, true, 1, None);
        let lesson_id = lessons[0].id;
        server.deref_mut().mock_facts_success(
            lesson_id,
            mock_facts(lesson_id, 5),
            2,
            true,
            1,
            None,
        );

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let _ = domain.get_facts(lesson_id, 0).await;
        let key = format!("FACTS_LAST_SYNC_TIME_{}", lesson_id);
        let settings = domain.provider.setting_repository.clone();

        let r = await_condition(
            || async {
                let r = settings.get_timestamp(&key).await;
                if r.is_none() {
                    log::debug!("r: none");
                } else {
                    log::debug!("r: {}", r.unwrap());
                }
                r
            },
            |timestamp| timestamp.is_some(),
        )
        .await;

        assert!(r.is_ok());
        assert!(r.unwrap().is_some()) // A timestamp should now exist
    }

    #[serial]
    #[tokio::test]
    async fn test_fact_sync_uses_saved_timestamp() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let lessons = mock_lessons(1);
        server
            .deref_mut()
            .mock_lessons_success(lessons.clone(), 0, true, 1, None);
        let lesson_id = lessons[0].id;
        server.deref_mut().mock_facts_success(
            lesson_id,
            mock_facts(lesson_id, 5),
            0,
            true,
            1,
            None,
        );

        let domain = fake_domain(server.url() + "/").await.unwrap();
        let settings = domain.provider.setting_repository.clone();

        // Insert a mock timestamp
        let key = format!("FACTS_LAST_SYNC_TIME_{lesson_id}");
        settings.put_timestamp(&key, 1337).await;

        let _ = domain
            .login("user".to_string(), "password".to_string())
            .await;

        let r = await_condition(
            || async {
                let r = domain.get_facts(lesson_id, 0).await;
                r.unwrap().len()
            },
            |count| *count == 5,
        )
        .await;

        assert!(r.is_ok());
        assert_eq!(5, r.unwrap()); // Expect only the "updated" facts
    }
}
