use crate::data::api::Api;
//use anyhow::{bail, Ok};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct LoginResponse {
    pub(super) access: String,
    pub(super) refresh: String,
}

#[derive(Debug, Error, Clone)]
pub(super) enum TokenApiError {
    #[error("username and/or password incorrect.")]
    Unauthorised(),
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl From<reqwest::Error> for TokenApiError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        TokenApiError::Unexpected(format!("{value:?}"))
    }
}

pub(super) trait TokenApi {
    async fn login(
        &self, username: String, password: String,
    ) -> Result<LoginResponse, TokenApiError>;
}

const LOGIN_URL: &str = "jwt/create";

impl TokenApi for Api {
    async fn login(
        &self, username: String, password: String,
    ) -> Result<LoginResponse, TokenApiError> {
        let response = self
            .post(LOGIN_URL.to_string())
            .form(&[("username", username), ("password", password)])
            .send()
            .await?;
        return match response.status() {
            StatusCode::OK => {
                let r = response.json::<LoginResponse>().await?;
                Ok(r)
            }
            StatusCode::UNAUTHORIZED => Err(TokenApiError::Unauthorised()),
            _ => Err(TokenApiError::Unauthorised()),
        };
    }
}
