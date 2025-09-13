use crate::data::api::{AuthApi, PagedResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type LessonsResponse = PagedResponse<LessonResponse>;

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
    async fn get_lessons(
        &self,
        page_no: u8,
        updated_after: Option<u64>,
    ) -> reqwest::Result<LessonsResponse>;
}

const LESSONS_URL: &str = "lessons";

impl LessonsApi for AuthApi {
    async fn get_lessons(
        &self,
        page_no: u8,
        updated_after: Option<u64>,
    ) -> reqwest::Result<LessonsResponse> {
        let mut params: Vec<(String, String)> = Vec::new();
        params.push(("page_no".to_string(), (page_no + 1).to_string())); // Api is 1 based
        if let Some(updated_after) = updated_after {
            log::trace!("Adding header param {}", &updated_after.to_string());
            params.push(("since".to_string(), updated_after.to_string()));
        }
        let iter = params.iter();
        let r = self
            .get(LESSONS_URL.to_string(), Some(iter))
            .await
            .send()
            .await?
            .json::<LessonsResponse>()
            .await?;
        Ok(r)
    }
}
