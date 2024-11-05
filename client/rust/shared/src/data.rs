pub(crate) mod auth;
pub(crate) mod lessons;
pub(crate) mod settings;
pub(crate) mod db;
pub(crate) mod api;

use std::result::Result;
use std::sync::Arc;
use std::thread::yield_now;

use api::{Api, AuthApi};
use auth::manager::SessionManager;
use crate::domain::auth::Session;
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
        loop {
            log::trace!("loop");
            let manager = self.session_manager.clone();
            let mut manager = manager.lock().await;
            let repository = self.lesson_repository.clone();
            let mut repository = repository.lock().await;

            let changed = manager.state.changed().await.is_ok();
            
            if changed {
                //let session_manager = safe_get!(self.session_manager);
                let session = manager.state.borrow();
                let session = session.clone();

                log::trace!("Got session: {:?}", session);
                if let Session::Authenticated(_) = session {
                    log::trace!("Starting lesson repo...");
                    repository.start();
                } else {
                    log::trace!("Stopping lesson repo...");
                    repository.stop();
                }
                log::trace!("done");
            }

            drop(manager);
            drop(repository);
            yield_now();
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
