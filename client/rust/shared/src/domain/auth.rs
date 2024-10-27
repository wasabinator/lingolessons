use uniffi::deps::log::trace;

use crate::{data::{api::Api, db::Db}, domain::Domain, ArcMutex};

use super::DomainResult;

/// Session domain model
#[derive(uniffi::Enum, PartialEq)]
#[derive(Clone)]
pub enum Session {
    None,
    Authenticated(String),
}

/// Manager the domain requires for managing the session
pub(crate) struct SessionManager {
    pub(crate) api: ArcMutex<Api>,
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
        let manager = self.provider.session_manager.lock().await;
        let session = manager.get_session().await?;
        Ok(session)
    }

    async fn login(&self, username: String, password: String) -> DomainResult<Session> {
        trace!("login");
        let manager = self.provider.session_manager.lock().await;
        let session = manager.login(username, password).await?;
        Ok(session)
    }

    async fn logout(&self) -> DomainResult<()> {
        trace!("logout");
        let manager = self.provider.session_manager.lock().await;
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
        let domain = fake_domain(server.url() + "/").unwrap();

        server.deref_mut().mock_login_success();

        let r = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_login_failure() {
        let mut server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url() + "/").unwrap();

        server.deref_mut().mock_login_failure();

        let r = domain.login("user".to_string(), "password".to_string()).await;
        assert!(r.is_err());
    }
}
