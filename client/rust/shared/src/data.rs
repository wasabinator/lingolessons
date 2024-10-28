pub(crate) mod auth;
pub(crate) mod lessons;
pub(crate) mod settings;
pub(crate) mod db;
pub(crate) mod api;

use std::result::Result;

use api::{Api, AuthApi};

use crate::domain::auth::SessionManager;
use crate::domain::lessons::LessonRepository;
use crate::domain::DomainError;
use crate::data::db::Db;
use crate::{arc_mutex, ArcMutex};

pub(crate) struct DataServiceProvider {
    pub(super) session_manager: ArcMutex<SessionManager>,
    pub(super) lesson_repository: ArcMutex<LessonRepository>,
}

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String,
        data_path: String
    ) -> Result<DataServiceProvider, DomainError> {
        let api = arc_mutex(Api::new(base_url)?);
        let db = arc_mutex(Db::open(data_path)?);
        let session_manager = arc_mutex(SessionManager::new(api.clone(), db.clone()));
        let auth_api = arc_mutex(AuthApi::new(api.clone(), session_manager.clone()));
        let lesson_repository = arc_mutex(LessonRepository::new(auth_api.clone(), db.clone()));
        Ok(Self {
            session_manager,
            lesson_repository,
        })
    }
}
