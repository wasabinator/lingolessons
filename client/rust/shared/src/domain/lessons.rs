use chrono::{DateTime, Local};
use uniffi::deps::log::trace;
use uuid::Uuid;

use crate::{data::{api::Api, db::Db}, domain::Domain, ArcMutex};

use super::DomainResult;

#[derive(uniffi::Record, PartialEq)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub r#type: LessonType,
    pub language1: String,
    pub language2: String,
    pub owner: String,
    pub updated_at: DateTime<Local>,
}

#[derive(uniffi::Enum, PartialEq)]
pub enum LessonType {
    Vocabulary,
    Grammar,
}

pub trait Lessons {
    fn get_lessons(&self) -> impl std::future::Future<Output = DomainResult<Vec<Lesson>>> + Send;
    fn get_lesson(&self, id: String) -> impl std::future::Future<Output = DomainResult<Option<Lesson>>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Lessons for Domain {
    async fn get_lessons(&self) -> DomainResult<Vec<Lesson>> {
        trace!("get_lessons");
        Ok(Vec::new())
    }

    async fn get_lesson(&self, id: String) -> DomainResult<Option<Lesson>> {
        Ok(None)
    }
}

/// Repository the domain requires for getting and updating lessons
pub(crate) struct LessonRepository {
    pub(crate) api: ArcMutex<Api>,
    pub(crate) db: ArcMutex<Db>,
}
