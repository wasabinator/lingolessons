use crate::{data::DataServiceProvider, domain::auth::AuthError};
use std::sync::Arc;

pub mod auth;
mod common;
pub mod lessons;
pub mod runtime;
pub mod settings;

pub trait DomainRule {}

#[derive(Debug, PartialEq, thiserror::Error, uniffi::Error, Clone)]
pub enum DomainError {
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("Database: {0}")]
    Database(String),
    #[error("Api: {0}")]
    Api(String),
    #[error("Auth: {0}")]
    Auth(AuthError),
}

pub type DomainResult<T = ()> = anyhow::Result<T, DomainError>;

#[derive(uniffi::Object, Clone)]
pub struct Domain {
    provider: Arc<DataServiceProvider>,
}

#[derive(uniffi::Object, Clone)]
pub struct DomainBuilder {
    _data_path: Option<String>,
    _base_url: Option<String>,
}

#[uniffi::export]
#[allow(clippy::new_without_default)]
impl DomainBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self { _data_path: None, _base_url: None }
    }

    pub fn data_path(&self, path: String) -> Self {
        let mut builder = self.clone();
        builder._data_path = Some(path);
        builder
    }

    pub fn base_url(&self, url: String) -> Self {
        let mut builder = self.clone();
        builder._base_url = Some(url);
        builder
    }

    pub fn build(&self) -> Result<Domain, DomainError> {
        let base_url = self._base_url.clone().expect("base_url was missing");
        let data_path = self._data_path.clone().expect("data_path was missing");

        init();

        Ok(Domain { provider: Arc::new(DataServiceProvider::new(base_url, data_path)?) })
    }
}

fn init() {
    let _ = env_logger::try_init();
    log::trace!("Initialising domain");

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default().with_max_level(uniffi::deps::log::LevelFilter::Debug),
        );
    }
}

#[uniffi::export]
impl Domain {
    // Implementation filled in via feature-specific traits
}

#[cfg(test)]
pub(crate) async fn fake_domain(base_url: String) -> Result<Domain, DomainError> {
    init();

    Ok(Domain {
        provider: Arc::new(DataServiceProvider::new(
            base_url,
            "fake_path".to_string(), // Unimportant path as not used with the in memory test db
        )?),
    })
}

#[cfg(test)]
mod tests {
    use crate::domain::fake_domain;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_mock_ctor() {
        let server = mockito::Server::new_async().await;
        let domain = fake_domain(server.url()).await;
        assert!(domain.is_ok());
    }
}
