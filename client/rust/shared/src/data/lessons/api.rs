use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::api::Api;

// {
//     "count": 11,
//     "next": "http://127.0.0.1:8000/api/v1/lessons/?page=9&page_size=1",
//     "previous": "http://127.0.0.1:8000/api/v1/lessons/?page=7&page_size=1",
//     "results": [
//         {
//             "id": "4",
//             "title": "aaa",
//             "type": 0,
//             "language1": 1,
//             "language2": 3,
//             "owner": "admin",
//             "updated_at": 1720705788
//         }
//     ]
// }

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
    pub updated_at: u128,
}
