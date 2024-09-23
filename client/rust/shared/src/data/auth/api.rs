use serde::Deserialize;

use crate::data::api::Api;

#[derive(Deserialize, Debug)]
pub(super) struct LoginResponse {
    pub(super) access: String,
    pub(super) refresh: String,
}

pub(super) trait TokenApi {
    async fn login(&self, username: String, password: String) -> reqwest::Result<LoginResponse>;
}

impl TokenApi for Api {
    async fn login(&self, username: String, password: String) -> reqwest::Result<LoginResponse> {
        let r = self.post("jwt/create".to_string())
            .form(&[("username", username), ("password", password)])
            .send().await?
            .json::<LoginResponse>()
            .await?;
        Ok(r)
    }
}
