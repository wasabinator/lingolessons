use chrono::{DateTime, Local};
use uniffi::deps::log::trace;

use crate::{data::{api::Api, db::Db}, domain::Domain, ArcMutex};

use super::DomainResult;

/// Lesson domain model
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

/// Lesson type domain model
#[derive(uniffi::Enum, PartialEq)]
pub enum LessonType {
    Vocabulary,
    Grammar,
}

/// Repository the domain requires for getting and updating lessons
pub(crate) struct LessonRepository {
    pub(crate) api: ArcMutex<Api>,
    pub(crate) db: ArcMutex<Db>,
}

pub trait Lessons {
    fn get_lessons(&self) -> impl std::future::Future<Output = DomainResult<Vec<Lesson>>> + Send;
    fn get_lesson(&self, id: String) -> impl std::future::Future<Output = DomainResult<Option<Lesson>>> + Send;
}

#[uniffi::export(async_runtime = "tokio")]
impl Lessons for Domain {
    async fn get_lessons(&self) -> DomainResult<Vec<Lesson>> {
        trace!("get_lessons");
        let repository = self.provider.lesson_repository.lock().await;
        let lessons = repository.get_lessons().await?;
        Ok(lessons)
    }

    async fn get_lesson(&self, _id: String) -> DomainResult<Option<Lesson>> {
        trace!("get_lesson");
        Ok(None) // TODO: Implement when client has a need to fetch lesson details
    }
}
