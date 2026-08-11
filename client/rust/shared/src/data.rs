pub(crate) mod api;
#[cfg(test)]
pub(crate) mod api_mocks;
pub(crate) mod auth;
pub(crate) mod db;
pub(crate) mod facts;
pub(crate) mod lessons;
pub(crate) mod settings;

use crate::{
    data::db::Db,
    domain::{
        auth::SessionManager,
        facts::FactRepository,
        lessons::LessonRepository,
        settings::SettingRepository,
        DomainError,
    },
};
use api::{Api, AuthApi};
use std::rc::Rc;
use tokio_util::task::LocalPoolHandle;

pub(crate) struct DataServiceProvider {
    /// Handle back onto the runtime worker thread, used to hop off the thread when unsubscribing.
    pub(crate) pool: LocalPoolHandle,
    pub(super) session_manager: Rc<SessionManager>,
    pub(super) lesson_repository: Rc<LessonRepository>,
    pub(super) fact_repository: Rc<FactRepository>,
    #[allow(unused)]
    pub(super) setting_repository: Rc<SettingRepository>,
    #[allow(unused)]
    service_manager: Rc<DataServiceManager>,
}

#[allow(dead_code)]
struct DataServiceManager {
    session_manager: Rc<SessionManager>,
    lesson_repository: Rc<LessonRepository>,
    fact_repository: Rc<FactRepository>,
}

impl DataServiceManager {
    fn new(
        session_manager: Rc<SessionManager>,
        lesson_repository: Rc<LessonRepository>,
        fact_repository: Rc<FactRepository>,
    ) -> Self {
        Self {
            session_manager,
            lesson_repository,
            fact_repository,
        }
    }
}

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String,
        data_path: String,
        pool: LocalPoolHandle,
    ) -> Result<DataServiceProvider, DomainError> {
        let api = Rc::new(Api::new(base_url)?);
        let db = Rc::new(Db::open(data_path)?);

        let session_manager = Rc::new(SessionManager::new(api.clone(), db.clone()));

        let settings = Rc::new(SettingRepository::new(db.clone()));

        let auth_api = Rc::new(AuthApi::new(api.clone(), session_manager.clone()));
        let fact_repository = Rc::new(FactRepository::new(
            auth_api.clone(),
            db.clone(),
            settings.clone(),
        ));
        let lesson_repository = Rc::new(LessonRepository::new(auth_api, db, settings.clone()));

        let service_manager = Rc::new(DataServiceManager::new(
            session_manager.clone(),
            lesson_repository.clone(),
            fact_repository.clone(),
        ));

        Ok(Self {
            pool,
            session_manager,
            lesson_repository,
            fact_repository,
            setting_repository: settings,
            service_manager,
        })
    }
}
