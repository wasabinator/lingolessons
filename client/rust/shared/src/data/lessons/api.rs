use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::api::AuthApi;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct LessonsResponse {
    pub count: u16,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<LessonResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct LessonResponse {
    pub id: Uuid,
    pub title: String,
    pub r#type: u8,
    pub language1: String,
    pub language2: String,
    pub owner: String,
    pub is_deleted: bool,
    pub updated_at: i64,
}

pub(super) trait LessonsApi {
    async fn get_lessons(&self, page_no: u8) -> reqwest::Result<LessonsResponse>;
}

const LESSONS_URL: &str = "lessons";

impl LessonsApi for AuthApi {
    async fn get_lessons(&self, page_no: u8) -> reqwest::Result<LessonsResponse> {
        let params = [
            ("page_no".to_string(), (page_no + 1).to_string()), // Api is 1 based
        ];
        let iter = params.iter();
        let r = self.get(LESSONS_URL.to_string(), Some(iter)).await
            .send().await?
            .json::<LessonsResponse>()
            .await?;
        Ok(r)
    }
}
