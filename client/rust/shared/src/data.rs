pub(crate) mod auth;
pub(crate) mod lessons;
pub(crate) mod settings;
pub(crate) mod db;
pub(crate) mod api;

use std::borrow::BorrowMut;
use std::result::Result;

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
    _service_manager: DataServiceManager,
}

struct DataServiceManager {
    runtime: Runtime,
    lesson_repository: ArcMutex<LessonRepository>,
}

const SERVICE_MANAGER_TASK: &str = "SERVICE_MANAGER_TASK";

impl DataServiceManager {
    async fn new(
        runtime: Runtime,
        session_manager: ArcMutex<SessionManager>,
        lesson_repository: ArcMutex<LessonRepository>,
    ) -> Self {
        let mut manager = DataServiceManager {
            runtime: runtime,
            lesson_repository: lesson_repository.clone(),
        };
        manager.borrow_mut().start(
            &session_manager.lock().await.state.subscribe()
        ).await;
        manager
    }

    async fn start(
        &mut self, 
        session_rx: &tokio::sync::watch::Receiver<Session>,
    ) {
        let mut session_rx = session_rx.to_owned();
        let lesson_repository = self.lesson_repository.clone();

        let _ = self.runtime.spawn(
            SERVICE_MANAGER_TASK.into(), 
            async move {
                let l = lesson_repository.clone();
                while session_rx.changed().await.is_ok() {
                    let state = session_rx.borrow().clone();
                    if let Session::Authenticated(_) = state {
                        l.lock().await.start();
                    } else {
                        lesson_repository.lock().await.stop();
                    }
                }
            }
        );
    }
} 

impl DataServiceProvider {
    pub(crate) async fn new(
        base_url: String,
        data_path: String
    ) -> Result<DataServiceProvider, DomainError> {
        let api = arc_mutex(Api::new(base_url)?);
        let db = arc_mutex(Db::open(data_path)?);
        let session_manager = arc_mutex(SessionManager::new(api.clone(), db.clone()).await);
        let auth_api = arc_mutex(AuthApi::new(api.clone(), session_manager.clone()));
        let lesson_repository = arc_mutex(LessonRepository::new(
            Runtime::new(),
            auth_api.clone(), 
            db.clone()
        ));

        let service_manager = DataServiceManager::new(
            Runtime::new(),
            session_manager.clone(),
            lesson_repository.clone(),
        ).await;

        Ok(Self {
            session_manager: session_manager.clone(),
            lesson_repository: lesson_repository.clone(),
            _service_manager: service_manager,
        })
    }
}
