
use reqwest::RequestBuilder;
use concat_string::concat_string;

use crate::{domain::{auth::SessionManager, DomainError, DomainResult}, ArcMutex};

pub(crate) struct Api {
    base_url: String,
    client: reqwest::Client,
}

impl From<reqwest::Error> for DomainError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        DomainError::Api(format!("{:?}", value))
    }
}

impl Api {
    pub(super) fn new(base_url: String) -> DomainResult<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Api {
            base_url,
            client,
        })
    }

    pub(super) fn get(&self, url: String) -> RequestBuilder {
        self.client.get(concat_string!(self.base_url, url))
    }

    pub(super) fn post(&self, url: String) -> RequestBuilder {
        self.client.post(concat_string!(self.base_url, url))
    }
}

pub(crate) struct AuthApi {
    api: ArcMutex<Api>,
    session_manager: ArcMutex<SessionManager>,
}

impl AuthApi {
    pub(super) fn new(api: ArcMutex<Api>, session_manager: ArcMutex<SessionManager>) -> Self {
        AuthApi {
            api,
            session_manager,
        }
    }

    pub(super) async fn get(&self, url: String) -> RequestBuilder {
        println!("get");
        let api = self.api.lock().await;
        let session_manager = self.session_manager.lock().await;

        session_manager.decorate(api.get(url)).await
    }

    #[allow(dead_code)]
    pub(super) async fn post(&self, url: String) -> RequestBuilder {
        let api = self.api.lock().await;
        let session_manager = self.session_manager.lock().await;

        session_manager.decorate(api.post(url)).await
    }
}
