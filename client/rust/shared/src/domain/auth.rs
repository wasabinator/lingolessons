use uniffi::deps::log::trace;

use crate::{data::{api::Api, db::Db}, domain::Domain, ArcMutex};
//use async_compat::Compat;

use super::DomainResult;

#[derive(uniffi::Enum, PartialEq)]
#[derive(Clone)]
pub enum Session {
    None,
    Authenticated(String),
}

pub trait Auth {
    async fn get_session(&self) -> DomainResult<Session>;
    async fn login(&self, username: String, password: String) -> DomainResult<Session>;
    async fn logout(&self) -> DomainResult<()>;
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
        let manager = self.provider.session_manager.lock().await;
        let session = manager.login(username, password).await?;
        Ok(session)
    }

    async fn logout(&self) -> DomainResult<()> {
        let manager = self.provider.session_manager.lock().await;
        manager.logout().await?;
        Ok(())
    }
}

pub(crate) struct SessionManager {
    pub(crate) api: ArcMutex<Api>,
    pub(crate) db: ArcMutex<Db>,
}

#[cfg(test)]
mod tests {
    use crate::domain::fake_domain;

    use super::*;

    #[tokio::test]
    async fn test_callback() {
        env_logger::init();

        let domain = fake_domain();
        let binding = domain.unwrap();
        let r = binding.login("admin".to_string(), "admin".to_string()).await;
        assert!(r.is_ok());
    }
}
