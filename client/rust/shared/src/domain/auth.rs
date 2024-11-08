use std::sync::Arc;

use uniffi::deps::log::trace;

use crate::{data::{api::Api, db::Db}, domain::Domain, ArcMutex};

use super::{runtime::Runtime, DomainResult};

/// Session domain model
#[derive(uniffi::Enum, PartialEq)]
#[derive(Clone, Debug)]
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
    pub(crate) db: ArcMutex<Db>,
}

pub trait Auth {
    fn get_session(&self) -> impl std::future::Future<Output = DomainResult<Session>> + Send;
    fn login(&self, username: String, password: String) -> impl std::future::Future<Output = DomainResult<Session>> + Send;
    fn logout(&self) -> impl std::future::Future<Output = DomainResult<()>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Auth for Domain {
    async fn get_session(&self) -> DomainResult<Session> {
        trace!("get_session");
        let provider = self.provider.clone();
        trace!("get_session - domain thread");
        let manager = provider.session_manager.clone();

        let manager = manager.lock().await;
        trace!("got manager");
        let session = manager.state.borrow();
        Ok(session.clone())
    }

    async fn login(&self, username: String, password: String) -> DomainResult<Session> {
        trace!("login");
        let provider = self.provider.clone();

        trace!("login - domain thread");
        let manager = provider.session_manager.clone();

        trace!("got manager");
        let manager = manager.lock().await;
        let session = manager.login(username, password).await?;
        Ok(session)
    }

    async fn logout(&self) -> DomainResult<()> {
        let provider = self.provider.clone();
        trace!("logout");
        let manager = provider.session_manager.clone();
        let mut manager = manager.lock().await;
        manager.logout().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use crate::{data::auth::api_mocks::TokenApiMocks, domain::fake_domain};

    use super::*;

    #[tokio::test]
    async fn test_login_success() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_login_success();

        let r = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r.is_ok());
        let s = domain.get_session().await;
        assert!(s.is_ok());
        assert_eq!(Session::Authenticated("user".into()), s.unwrap());
    }

    #[tokio::test]
    async fn test_login_failure() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_login_failure();

        let r = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r.is_err());
    }
}
