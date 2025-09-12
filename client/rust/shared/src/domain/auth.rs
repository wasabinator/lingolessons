use super::{runtime::Runtime, DomainResult};
use crate::{
    data::{api::Api, db::Db},
    domain::Domain,
};
use std::sync::Arc;
use thiserror::Error;
use uniffi::deps::log::trace;

/// Errors produced by this domain
#[derive(uniffi::Enum, Debug, Error, PartialEq, Clone)]
pub enum AuthError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
}

/// Session domain model
#[derive(uniffi::Enum, PartialEq, Clone, Debug)]
pub enum Session {
    None,
    Authenticated(String),
}

/// Manager the domain requires for managing the session
pub(crate) struct SessionManager {
    pub(crate) runtime: Runtime,
    pub(crate) state_mut: tokio::sync::watch::Sender<Session>,
    pub(crate) state: tokio::sync::watch::Receiver<Session>,
    pub(crate) api: Arc<Api>,
    pub(crate) db: Arc<Db>,
}

pub trait Auth {
    fn get_session(&self) -> impl std::future::Future<Output = DomainResult<Session>> + Send;
    fn login(
        &self, username: String, password: String,
    ) -> impl std::future::Future<Output = DomainResult<Session>> + Send;
    fn logout(&self) -> impl std::future::Future<Output = DomainResult<()>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Auth for Domain {
    async fn get_session(&self) -> DomainResult<Session> {
        trace!("get_session");
        let provider = self.provider.clone();
        trace!("get_session - domain thread");
        let manager = provider.session_manager.clone();

        let session = manager.state.borrow();
        Ok(session.clone())
    }

    async fn login(&self, username: String, password: String) -> DomainResult<Session> {
        trace!("login");
        let provider = self.provider.clone();

        trace!("login - domain thread");
        let manager = provider.session_manager.clone();

        trace!("got manager");
        let session = manager.login(username, password).await?;
        Ok(session)
    }

    async fn logout(&self) -> DomainResult<()> {
        let provider = self.provider.clone();
        trace!("logout");
        let manager = provider.session_manager.clone();
        manager.logout().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            auth::api_mocks::TokenApiMocks,
            facts::api_mocks::FactsApiMocks,
            lessons::api_mocks::{mock_lessons, LessonApiMocks},
        },
        domain::{fake_domain, DomainError},
    };
    use serial_test::serial;
    use std::ops::DerefMut;

    #[serial]
    #[tokio::test]
    async fn test_login_success() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let mock_lessons = mock_lessons(1);
        server.deref_mut().mock_facts_success(mock_lessons[0].id, Vec::new(), 0, true, 1, None);
        server.deref_mut().mock_lessons_success(mock_lessons, 0, true, 1, None);

        let domain = fake_domain(server.url() + "/").await.unwrap();

        let r = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r.is_ok());
        let s = domain.get_session().await;
        assert!(s.is_ok());
        assert_eq!(Session::Authenticated("user".into()), s.unwrap());
    }

    #[serial]
    #[tokio::test]
    async fn test_login_failure() {
        let mut server = mockito::Server::new_async().await;

        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_login_http_failure(401);
        let r1 = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r1.is_err());
        assert_eq!(DomainError::Auth(AuthError::InvalidCredentials), r1.unwrap_err());

        server.deref_mut().mock_login_http_failure(500);
        let r2 = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r2.is_err());
        assert!(matches!(r2, Err(DomainError::Api(_))));

        server.deref_mut().mock_login_other_failure();
        let r3 = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r3.is_err());
        assert!(matches!(r3, Err(DomainError::Api(_))));
    }
}
