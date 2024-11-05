use std::{borrow::BorrowMut, sync::Arc};

use reqwest::RequestBuilder;

use crate::{data::{api::Api, db::Db, Runtime}, domain::DomainError, ArcMutex};
use crate::domain::auth::Session;
use super::api::TokenApi;

use super::db::TokenDao;

const SESSION_MANAGER_INIT_TASK: &str = "SESSION_MANAGER_INIT_TASK";

/// Manager the domain requires for managing the session
pub(crate) struct SessionManager {
    runtime: Runtime,
               state_mut: tokio::sync::watch::Sender<Session>,
    pub(crate) state: tokio::sync::watch::Receiver<Session>,
    pub(crate) api: Arc<Api>,
    pub(crate) db: ArcMutex<Db>,
}

// All branches into session manager arrive via the domain thread
//unsafe impl Send for SessionManager {}

impl SessionManager {
    pub(in crate::data) fn new(
        api: Arc<Api>, 
        db: ArcMutex<Db>
    ) -> Self {
        let _db = db.clone();

        let (tx, rx) = tokio::sync::watch::channel(Session::None);
        let mut manager = SessionManager {
            runtime: Runtime::new(),
            state: rx,
            state_mut: tx,
            api: api.clone(),
            db: db.clone()
        };

        manager.borrow_mut().start();
        manager
    }

    fn start(&mut self) {
        log::trace!("Starting session manager");
        let db = self.db.clone();
        let state_mut = self.state_mut.to_owned();

        self.runtime.borrow_mut().spawn(SESSION_MANAGER_INIT_TASK.into(), async move {
            log::trace!("Fetching session from db");
            let session = match db.lock().await.get_token() { 
                Ok(token) => token.map_or(Session::None, |token| {
                    Session::Authenticated(token.username)
                }),
                Err(_) => Session::None,
            };
            log::trace!("Initial Session from database {:?}", session);
            let _ = state_mut.send(session.clone());
        });
    }

    pub(crate) async fn login(&self, username: String, password: String) -> anyhow::Result<Session, DomainError> {
        log::trace!("session_manager::login()");
        let api = self.api.clone();
        let session = api.login(username.clone(), password).await?;
        log::trace!("api::Login response {:?}", session);
        let db = self.db.clone();
        db.lock().await.set_token(username.clone(), session.access, session.refresh)?;
        let session = Session::Authenticated(username);
        log::trace!("New session: {:?}", session);
        let r = self.state_mut.send(session.clone());
        log::trace!("rc: {:?}", r);
        anyhow::Result::Ok(session)
    }

    pub(crate) async fn logout(&mut self) -> uniffi::Result<(), DomainError> {
        let db = self.db.clone();
        db.lock().await.del_token()?;
        Ok(())
    }

    pub(crate) async fn decorate(&self, builder: RequestBuilder) -> RequestBuilder {
        let db = self.db.lock().await;
        match db.get_token() {
            Ok(session) => match session {
                Some(token) => {
                    #[cfg(test)]
                    log::trace!("Using test session token: {}", token.auth_token);
                    builder.bearer_auth(token.auth_token)
                },
                None => builder
            },
            Err(e) => {
                log::error!("{:?}", e);
                builder
            }
        }
    }
}
