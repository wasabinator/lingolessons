use std::sync::Arc;

use uniffi::deps::log::info;

use crate::data::DataServiceProvider;

pub mod auth;

#[derive(Debug, PartialEq, thiserror::Error, uniffi::Error)]
pub enum DomainError {
    Unexpected(String),
    Database(String),
    Api(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            DomainError::Unexpected(ref s) => s,
            DomainError::Database(ref s) => s,
            DomainError::Api(ref s) => s,
        })
    }
}

pub type DomainResult<T = ()> = Result<T, DomainError>;

#[derive(uniffi::Object)]
#[derive(Clone)]
pub struct Domain {
    provider: Arc<DataServiceProvider>
}

#[derive(uniffi::Object)]
#[derive(Clone)]
pub struct DomainBuilder {
    _data_path: Option<String>,
    _base_url: Option<String>,
}

#[uniffi::export]
impl DomainBuilder {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            _data_path: None,
            _base_url: None,
        }
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

        Ok(
            Domain {
                provider: Arc::new(DataServiceProvider::new(base_url, data_path)?)
            }
        )
    }
}

fn init() {
    env_logger::init();
    info!("Initialising domain");

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(uniffi::deps::log::LevelFilter::Debug),
        );
    }
}

#[uniffi::export]
impl Domain {
    // Implementation filled in via feature-specific traits
}

#[cfg(test)]
pub(crate) fn fake_domain() -> Result<Domain, DomainError> {
    Ok(
        Domain {
            provider: Arc::new(DataServiceProvider::new(
                "http://127.0.0.1:8000/api/v1/".to_string(),
                "fake_path".to_string()
            )?)
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::domain::fake_domain;

    #[tokio::test]
    async fn test() {
        let domain = fake_domain();
        assert!(domain.is_ok());
    }
}