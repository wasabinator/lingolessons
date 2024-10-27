use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub type_id: u8,
    pub language1: u8,
    pub language2: u8,
    pub owner: String,
    pub updated_at: i64,
}
