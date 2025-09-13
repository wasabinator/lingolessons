use crate::data::api::{AuthApi, PagedResponse};
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type FactsResponse = PagedResponse<FactResponse>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct FactResponse {
    pub id: Uuid,
    pub element1: String,
    pub element2: String,
    pub hint: String,
    pub is_deleted: bool,
    pub updated_at: i64,
}

pub(super) trait FactsApi {
    async fn get_facts(
        &self,
        lesson_id: Uuid,
        page_no: u8,
        updated_after: Option<u64>,
    ) -> reqwest::Result<FactsResponse>;
}

const FACTS_URL: &str = "facts";

impl FactsApi for AuthApi {
    async fn get_facts(
        &self,
        lesson_id: Uuid,
        page_no: u8,
        updated_after: Option<u64>,
    ) -> reqwest::Result<FactsResponse> {
        let mut params: Vec<(String, String)> = Vec::new();
        debug!("!!!!!!!!!!!!!!!!!!!! lesson_id: {lesson_id}, page_no: {page_no}");
        params.push(("lesson_id".to_string(), lesson_id.to_string()));
        params.push(("page_no".to_string(), (page_no + 1).to_string())); // Api is 1 based
        if let Some(updated_after) = updated_after {
            log::trace!("Adding header param {}", &updated_after.to_string());
            params.push(("since".to_string(), updated_after.to_string()));
        }
        let iter = params.iter();
        let r = self
            .get(FACTS_URL.to_string(), Some(iter))
            .await
            .send()
            .await?
            .json::<FactsResponse>()
            .await?;
        Ok(r)
    }
}
