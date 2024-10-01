
use reqwest::RequestBuilder;
use concat_string::concat_string;

use crate::domain::{DomainError, DomainResult};

pub(crate) struct Api {
    base_url: String,
    client: reqwest::Client,
}

impl From<reqwest::Error> for DomainError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        DomainError::Api(value.to_string())
    }
}

impl Api {
    pub(super) fn new(base_url: String) -> DomainResult<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Api {
            base_url: base_url,
            client: client
        })
    }

    pub(super) fn post(&self, url: String) -> RequestBuilder {
        self.client.post(concat_string!(self.base_url, url))
    }
}
