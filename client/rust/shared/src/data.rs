pub(crate) mod auth;
pub(crate) mod lessons;
pub(crate) mod settings;
pub(crate) mod db;
pub(crate) mod api;

use std::result::Result;
use std::sync::Arc;
use std::thread::yield_now;

use api::{Api, AuthApi};
use crate::domain::auth::{Session, SessionManager};
use crate::domain::lessons::LessonRepository;
use crate::domain::runtime::Runtime;
use crate::domain::DomainError;
use crate::data::db::Db;
use crate::{arc_mutex, ArcMutex};

pub(crate) struct DataServiceProvider {
    pub(super) session_manager: ArcMutex<SessionManager>,
    pub(super) lesson_repository: ArcMutex<LessonRepository>,
    #[allow(dead_code)]
    service_manager: Arc<DataServiceManager>,
}

#[derive(Clone)]
struct DataServiceManager {
    session_manager: ArcMutex<SessionManager>,
    lesson_repository: ArcMutex<LessonRepository>,
}

impl DataServiceManager {
    fn new(
        session_manager: ArcMutex<SessionManager>,
        lesson_repository: ArcMutex<LessonRepository>,
    ) -> Self {
        let manager = DataServiceManager {
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
        };
        manager.clone().start();
        manager
    }

    fn start(self) {
        tokio::task::spawn(
            async move { self.run().await }
        );
    }

    async fn run(&self) {
        log::trace!("DataServiceManager::run()");
        let manager = self.session_manager.clone();
        let manager = manager.lock().await;
        let mut state = manager.state.clone();
        drop(manager); // Drop the manager lock as we have cloned the state reference

        log::trace!("Beginning start change await loop");
        while state.changed().await.is_ok() {
            log::trace!("Received state change from session repo");

            let repository = self.lesson_repository.clone();

            // Obtain a short lived lock within this scope (will fall out of scope at the next iteration)
            let mut repository = repository.lock().await;

            let session = state.borrow();
            let session = session.clone();

            log::trace!("Got session: {:?}", session);
            if let Session::Authenticated(_) = session {
                log::trace!("Session Started - Stopping lesson repo...");
                repository.start();
            } else {
                log::trace!("Session Ended - Stopping lesson repo...");
                repository.stop();
            }

            log::trace!("Finished state loop, yielding then repeating");
            yield_now(); // Not strictly necessaty as the while loop await will yield anyway
        }
    }
} 

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String,
        data_path: String
    ) -> Result<DataServiceProvider, DomainError> {
        let api = Arc::new(Api::new(base_url)?);
        let db = arc_mutex(Db::open(data_path)?);
        
        let session_manager = arc_mutex(SessionManager::new(
            api.clone(), 
            db.clone()
        ));

        let auth_api = Arc::new(AuthApi::new(api.clone(), session_manager.clone()));
        let lesson_repository = arc_mutex(LessonRepository::new(
            Runtime::new(),
            auth_api, 
            db.clone()
        ));

        let service_manager = Arc::new(DataServiceManager::new(
            session_manager.clone(),
            lesson_repository.clone(),
        ));

        Ok(Self {
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
            service_manager: service_manager.clone(),
        })
    }
}
