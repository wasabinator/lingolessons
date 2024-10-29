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

#[derive(Serialize, Deserialize, Debug)]
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
    async fn get_lessons(&self) -> reqwest::Result<LessonsResponse>;
}

const LESSONS_URL: &str = "lessons";

impl LessonsApi for AuthApi {
    async fn get_lessons(&self) -> reqwest::Result<LessonsResponse> {
        let r = self.get(LESSONS_URL.to_string()).await
            .send().await?
            .json::<LessonsResponse>()
            .await?;
        Ok(r)
    }
}
