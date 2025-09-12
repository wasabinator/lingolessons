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
use lru::LruCache;
use std::{
    num::NonZeroUsize,
    result::Result,
    sync::{Arc, RwLock},
    thread::yield_now,
};

pub(crate) struct DataServiceProvider {
    pub(super) session_manager: Arc<SessionManager>,
    pub(super) lesson_repository: Arc<LessonRepository>,
    pub(super) fact_repository: Arc<FactRepository>,
    #[allow(unused)]
    pub(super) setting_repository: Arc<SettingRepository>,
    #[allow(unused)]
    service_manager: Arc<DataServiceManager>,
}

struct DataServiceManager {
    runtime: Runtime,
    session_manager: Arc<SessionManager>,
    lesson_repository: Arc<LessonRepository>,
    fact_repository: Arc<FactRepository>,
}

static MANAGER_START_TASK: &str = "MANAGER_START_TASK";

impl DataServiceManager {
    fn new(
        session_manager: Arc<SessionManager>, lesson_repository: Arc<LessonRepository>,
        fact_repository: Arc<FactRepository>,
    ) -> Self {
        let mut manager = DataServiceManager {
            runtime: Runtime::new(),
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
            fact_repository: fact_repository.clone(),
        };
        manager.start();
        manager
    }

    fn start(&mut self) {
        let session_manager = self.session_manager.clone();
        let lesson_repository = self.lesson_repository.clone();
        let fact_repository = self.fact_repository.clone();
        self.runtime.spawn(
            MANAGER_START_TASK.into(),
            Self::run(session_manager, lesson_repository, fact_repository),
        );
    }

    async fn run(
        session_manager: Arc<SessionManager>, lesson_repository: Arc<LessonRepository>,
        fact_repository: Arc<FactRepository>,
    ) {
        log::trace!("DataServiceManager::run()");
        let mut state = session_manager.state.clone();

        log::trace!("Beginning state change await loop");
        while state.changed().await.is_ok() {
            log::trace!("Received state change from session repo");

            let session = state.borrow().clone();
            let lesson_repo = lesson_repository.clone();
            let fact_repo = fact_repository.clone();

            if let Session::Authenticated(_) = session {
                log::trace!("Session Started - Stopping repos...");
                lesson_repo.start(fact_repo);
            } else {
                log::trace!("Session Ended - Stopping repos...");
                lesson_repo.stop();
                fact_repo.stop();
            }

            log::trace!("Finished state loop, yielding then repeating");
            yield_now(); // Not strictly necessity as the while loop await will yield anyway
        }
    }
}

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String, data_path: String,
    ) -> Result<DataServiceProvider, DomainError> {
        let api = Arc::new(Api::new(base_url)?);
        let db = Arc::new(Db::open(data_path)?);

        let session_manager = Arc::new(SessionManager::new(api.clone(), db.clone()));

        let settings = Arc::new(SettingRepository::new(db.clone()));

        let auth_api = Arc::new(AuthApi::new(api.clone(), session_manager.clone()));
        let fact_repository = Arc::new(FactRepository::new(
            Runtime::new(),
            auth_api.clone(),
            db.clone(),
            settings.clone(),
            RwLock::new(LruCache::new(NonZeroUsize::new(8).unwrap())),
        ));
        let lesson_repository = Arc::new(LessonRepository::new(
            Runtime::new(),
            auth_api,
            db,
            settings.clone(),
            RwLock::new(LruCache::new(NonZeroUsize::new(8).unwrap())),
            RwLock::new(LruCache::new(NonZeroUsize::new(20).unwrap())),
        ));

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
