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
        auth::{Session, SessionManager},
        facts::FactRepository,
        lessons::LessonRepository,
        runtime::Runtime,
        settings::SettingRepository,
        DomainError,
    },
};
use api::{Api, AuthApi};
use log::trace;
use std::{rc::Rc, result::Result, sync::Arc};

pub(crate) struct DataServiceProvider {
    pub(super) session_manager: Rc<SessionManager>,
    pub(super) lesson_repository: Rc<LessonRepository>,
    pub(super) fact_repository: Rc<FactRepository>,
    #[allow(unused)]
    pub(super) setting_repository: Rc<SettingRepository>,
    #[allow(unused)]
    service_manager: Rc<DataServiceManager>,
}

struct DataServiceManager {
    session_manager: Rc<SessionManager>,
    lesson_repository: Rc<LessonRepository>,
    fact_repository: Rc<FactRepository>,
}

/*
impl DataServiceManager {
    pub(super) fn new(
        session_manager: Rc<SessionManager>,
        lesson_repository: Rc<LessonRepository>,
        fact_repository: Rc<FactRepository>,
    ) -> Self {
        Self {
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
            fact_repository: fact_repository.clone(),
        }
    }

    // pub(super) fn start(&mut self) {
    //     let session_manager = self.session_manager.clone();
    //     let lesson_repository = self.lesson_repository.clone();
    //     let fact_repository = self.fact_repository.clone();
    //     self.runtime.spawn(
    //         MANAGER_START_TASK.into(),
    //         Self::run(session_manager, lesson_repository, fact_repository),
    //     );
    // }

    async fn run(
        session_manager: Rc<SessionManager>,
        lesson_repository: Rc<LessonRepository>,
        fact_repository: Rc<FactRepository>,
    ) {
        trace!("DataServiceManager::run()");
        let mut state = session_manager.state.clone();

        trace!("Beginning state change await loop");
        while state.changed().await.is_ok() {
            trace!("Received state change from session repo");

            let session = state.borrow().clone();
            let lesson_repo = lesson_repository.clone();
            let fact_repo = fact_repository.clone();

            if let Session::Authenticated(_) = session {
                trace!("Session Started - Stopping repos...");
                lesson_repo.start(fact_repo);
            } else {
                trace!("Session Ended - Stopping repos...");
                lesson_repo.stop();
                fact_repo.stop();
            }

            trace!("Finished state loop, yielding then repeating");
            yield_now(); // Not strictly necessity as the while loop await will yield anyway
        }
    }
}
*/

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String,
        data_path: String,
    ) -> Result<DataServiceProvider, DomainError> {
        let api = Arc::new(Api::new(base_url)?);
        let db = Arc::new(Db::open(data_path)?);

        let session_manager = Arc::new(SessionManager::new(api.clone(), db.clone()));

        let settings = Arc::new(SettingRepository::new(db.clone()));

        let auth_api = Arc::new(AuthApi::new(api.clone(), session_manager.clone()));
        let fact_repository = Arc::new(FactRepository::new(
            auth_api.clone(),
            db.clone(),
            settings.clone(),
        ));
        let lesson_repository = Arc::new(LessonRepository::new(auth_api, db, settings.clone()));

        let service_manager = Arc::new(DataServiceManager::new(
            session_manager.clone(),
            lesson_repository.clone(),
            fact_repository.clone(),
        ));

        Ok(Self {
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
            fact_repository: fact_repository.clone(),
            setting_repository: settings.clone(),
            service_manager: service_manager.clone(),
        })
    }
}
