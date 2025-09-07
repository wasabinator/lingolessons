use crate::{
    data::{api::AuthApi, db::Db},
    domain::{runtime::Runtime, settings::SettingRepository, Domain, DomainResult},
    ArcMutex, Run,
};
use log::trace;
use lru::LruCache;
use std::sync::Arc;
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
    pub(crate) db: ArcMutex<Db>,
    pub(crate) settings: ArcMutex<SettingRepository>,
    pub(crate) page_cache: LruCache<(Uuid, u8), Vec<Fact>>,
}

pub trait Facts {
    fn get_facts(
        &self, lesson_id: Uuid, page_no: u8,
    ) -> impl std::future::Future<Output = DomainResult<Vec<Fact>>> + Send;

    fn stop(&self) -> impl std::future::Future<Output = ()>;
}

#[uniffi::export(async_runtime = "tokio")]
impl Facts for Domain {
    async fn get_facts(&self, lesson_id: Uuid, page_no: u8) -> DomainResult<Vec<Fact>> {
        trace!("get_facts");

        let facts = self
            .provider
            .fact_repository
            .clone()
            .launch(|mut repo| async move { repo.get_facts(lesson_id, page_no).await })
            .await?;

        trace!("Received facts");
        Ok(facts)
    }

    async fn stop(&self) {
        //let repo = self.provider.lesson_repository.clone();
        //let mut r = repo.lock_owned().await;
        //r.stop();

        let repo = self.provider.fact_repository.clone();
        let mut r = repo.lock_owned().await;
        r.stop();
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
        // let mut server = mockito::Server::new_async().await;
        // let lesson_id = Uuid::new_v4();
        // server.deref_mut().mock_lessons_success(mock_lessons(5), 0, true, 1, None);
        // server.deref_mut().mock_facts_success(mock_facts(lesson_id, 5), 0, true, 1, None);
        // server.deref_mut().mock_login_success();

        // let domain = fake_domain(server.url() + "/").await.unwrap();
        // let _ = domain.login("user".to_string(), "password".to_string()).await;

        // // We wrap this check around a timeout
        // let r = await_condition(
        //     || async { domain.get_facts(lesson_id, 0).await.unwrap().len() },
        //     |count| *count == 5,
        // )
        // .await;

        // assert!(r.is_ok());
        // assert_eq!(5, r.unwrap());
    }

    // #[serial]
    // #[tokio::test]
    // async fn test_get_lessons_doesnt_persist_deleted_items() {
    //     let mut server = mockito::Server::new_async().await;
    //     server.deref_mut().mock_login_success();
    //     server.deref_mut().mock_lessons_success(mock_lessons(5), 1, true, 1, None);

    //     let domain = fake_domain(server.url() + "/").await.unwrap();
    //     let _ = domain.login("user".to_string(), "password".to_string()).await;

    //     let r = await_condition(
    //         || async { domain.get_lessons(0).await.unwrap().len() },
    //         |count| *count == 4,
    //     )
    //     .await;

    //     assert!(r.is_ok());
    //     assert_eq!(4, r.unwrap());
    // }

    // #[serial]
    // #[tokio::test]
    // async fn test_lesson_sync_saves_timestamp_on_success() {
    //     let mut server = mockito::Server::new_async().await;
    //     server.deref_mut().mock_login_success();
    //     server.deref_mut().mock_lessons_success(mock_lessons(10), 0, true, 1, None);

    //     let domain = fake_domain(server.url() + "/").await.unwrap();
    //     let _ = domain.login("user".to_string(), "password".to_string()).await;

    //     let _ = domain.get_lessons(0).await;

    //     let settings = domain.provider.setting_repository.clone();
    //     let r = await_condition(
    //         || async {
    //             settings
    //                 .launch(
    //                     |repo| async move { repo.get_timestamp("LESSONS_LAST_SYNC_TIME").await },
    //                 )
    //                 .await
    //         },
    //         |timestamp| timestamp.is_some(),
    //     )
    //     .await;

    //     assert!(r.is_ok());
    //     assert!(r.unwrap().is_some()) // A timestamp should now exist
    // }

    // #[serial]
    // #[tokio::test]
    // async fn test_lesson_sync_uses_saved_timestamp() {
    //     let mut server = mockito::Server::new_async().await;
    //     server.deref_mut().mock_login_success();
    //     server.deref_mut().mock_lessons_success(mock_lessons(2), 0, true, 1, Some(1337));

    //     let domain = fake_domain(server.url() + "/").await.unwrap();
    //     let settings = domain.provider.setting_repository.clone();

    //     // Insert a mock timestamp
    //     settings
    //         .launch(|repo| async move { repo.put_timestamp("LESSONS_LAST_SYNC_TIME", 1337).await })
    //         .await;

    //     let _ = domain.login("user".to_string(), "password".to_string()).await;

    //     let r = await_condition(
    //         || async {
    //             let r = domain.get_lessons(0).await;
    //             r.unwrap().len()
    //         },
    //         |count| *count == 2,
    //     )
    //     .await;

    //     assert!(r.is_ok());
    //     assert_eq!(2, r.unwrap()); // Expect only the "updated" lessons
    // }
}
