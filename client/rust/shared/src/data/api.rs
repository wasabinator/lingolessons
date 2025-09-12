use super::SessionManager;
use crate::domain::{DomainError, DomainResult};
use concat_string::concat_string;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Api {
    base_url: String,
    client: reqwest::Client,
}

impl From<reqwest::Error> for DomainError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        DomainError::Api(format!("{value:?}"))
    }
}

impl Api {
    pub(super) fn new(base_url: String) -> DomainResult<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Api { base_url, client })
    }

    pub(super) fn get(
        &self, url: String, params: Option<std::slice::Iter<(String, String)>>,
    ) -> RequestBuilder {
        let url = concat_string!(self.base_url, url);
        let url = match params {
            Some(params) => reqwest::Url::parse_with_params(url.as_str(), params),
            None => reqwest::Url::parse(url.as_str()),
        }
        .unwrap(); // TODO: This method should probably return Result<RequestBuilder> instead
        self.client.get(url)
    }

    pub(super) fn post(&self, url: String) -> RequestBuilder {
        self.client.post(concat_string!(self.base_url, url))
    }
}

pub(crate) struct AuthApi {
    api: Arc<Api>,
    session_manager: Arc<SessionManager>,
}

impl AuthApi {
    pub(super) fn new(api: Arc<Api>, session_manager: Arc<SessionManager>) -> Self {
        AuthApi { api, session_manager }
    }

    pub(super) async fn get(
        &self, url: String, params: Option<std::slice::Iter<'_, (String, String)>>,
    ) -> RequestBuilder {
        println!("about to obtain session manager lock to decorate the request");
        self.session_manager.decorate(self.api.get(url, params.clone())).await
    }

    #[allow(dead_code)]
    pub(super) async fn post(&self, url: String) -> RequestBuilder {
        let api = self.api.clone();

        self.session_manager.decorate(api.post(url)).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PagedResponse<T>
where
    T: Serialize,
{
    pub count: u16,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}
