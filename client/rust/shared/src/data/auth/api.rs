use crate::data::api::Api;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct LoginResponse {
    pub(super) access: String,
    pub(super) refresh: String,
}

pub(super) trait TokenApi {
    async fn login(&self, username: String, password: String) -> reqwest::Result<LoginResponse>;
}

const LOGIN_URL: &str = "jwt/create";

impl TokenApi for Api {
    async fn login(&self, username: String, password: String) -> reqwest::Result<LoginResponse> {
        let r = self
            .post(LOGIN_URL.to_string())
            .form(&[("username", username), ("password", password)])
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;
        Ok(r)
    }
}
