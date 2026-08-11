use super::DomainResult;
use crate::{
    data::{api::Api, db::Db, DataServiceProvider},
    domain::{
        common::{Broadcaster, Subscription},
        Domain,
    },
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use thiserror::Error;
use tokio::task::JoinHandle;
use uniffi::deps::log::trace;

/// Errors produced by this domain
#[derive(uniffi::Enum, Debug, Error, PartialEq, Clone)]
pub enum AuthError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
}

/// Session domain model
#[derive(uniffi::Enum, PartialEq, Clone, Debug)]
pub enum Session {
    None,
    Authenticated(String),
}

/// Manager the domain requires for managing the session
pub(crate) struct SessionManager {
    pub(crate) state_mut: tokio::sync::watch::Sender<Session>,
    pub(crate) state: tokio::sync::watch::Receiver<Session>,
    pub(crate) api: Rc<Api>,
    pub(crate) db: Rc<Db>,
}

#[derive(uniffi::Record, Debug)]
pub struct SessionSubscriptionResult {
    pub session: Session,
    pub subscription: Arc<Subscription>,
}

pub trait Auth {
    #[allow(async_fn_in_trait)]
    async fn get_session(
        &self,
        observer: Arc<dyn SessionObserver>,
    ) -> DomainResult<SessionSubscriptionResult>;
    fn login(
        &self,
        username: String,
        password: String,
    ) -> impl std::future::Future<Output = DomainResult<Session>> + Send;
    fn logout(&self) -> impl std::future::Future<Output = DomainResult<()>> + Send;
}

#[uniffi::export(with_foreign)]
pub trait SessionObserver: Send + Sync {
    fn on_change(&self, session: Session);
}

impl<F: Fn(Session) + Send + Sync> SessionObserver for F {
    fn on_change(&self, session: Session) {
        self(session)
    }
}

thread_local! {
    static SESSION_BROADCASTER: Rc<Broadcaster<dyn SessionObserver>> =
        Rc::new(Broadcaster::new());
}

thread_local! {
    static PUBLISHER_HANDLE: RefCell<Option<JoinHandle<()>>> = const { RefCell::new(None) };
}

#[uniffi::export(async_runtime = "tokio")]
impl Auth for Domain {
    async fn get_session(
        &self,
        observer: Arc<dyn SessionObserver>,
    ) -> DomainResult<SessionSubscriptionResult> {
        trace!("get_session");
        self.runtime
            .spawn("get_session".into(), async move |provider| {
                trace!("get_session - domain thread");
                SESSION_BROADCASTER.with(|b| b.subscribe(observer));

                start_session_publish_loop(&provider);

                let session = provider.session_manager.state.borrow().clone();
                let subscription = Arc::new(Subscription::new(provider.pool.clone(), || {
                    SESSION_BROADCASTER.with(|b| b.unsubscribe());
                }));

                Ok(SessionSubscriptionResult {
                    session,
                    subscription,
                })
            })
            .await?
    }

    async fn login(&self, username: String, password: String) -> DomainResult<Session> {
        trace!("login");
        self.runtime
            .spawn("login".into(), async move |provider| {
                let provider = provider.clone();

                trace!("login - data thread");
                let manager = provider.session_manager.clone();

                trace!("got manager");
                let session = manager.login(username, password).await?;
                Ok(session)
            })
            .await?
    }

    async fn logout(&self) -> DomainResult<()> {
        trace!("logout");
        self.runtime
            .spawn("logout".into(), async move |provider| {
                let provider = provider.clone();
                let manager = provider.session_manager.clone();
                manager.logout().await?;
                Ok(())
            })
            .await?
    }
}

/// Ensure the session watch loop is running. Started once at provider construction so
/// repositories react to session changes even before anyone subscribes; subsequent calls
/// are no-ops while the loop is alive.
pub(crate) fn start_session_publish_loop(provider: &Rc<DataServiceProvider>) {
    let is_running =
        PUBLISHER_HANDLE.with(|h| matches!(&*h.borrow(), Some(handle) if !handle.is_finished()));
    if !is_running {
        let provider = provider.clone();
        let handle = tokio::task::spawn_local(async move {
            run_session_publish_loop(provider).await;
        });
        PUBLISHER_HANDLE.with(|h| *h.borrow_mut() = Some(handle));
    }
}

async fn run_session_publish_loop(provider: Rc<DataServiceProvider>) {
    loop {
        let session_manager = provider.session_manager.clone();
        let mut state = session_manager.state.clone();

        trace!("Beginning state change await loop");
        while state.changed().await.is_ok() {
            trace!("Received state change from session repo");

            let session = state.borrow().clone();
            SESSION_BROADCASTER.with(|b| b.notify(|o| o.on_change(session)));

            let session = state.borrow().clone();

            if let Session::Authenticated(_) = session {
                trace!("Session Started - Stopping repos...");
                provider
                    .lesson_repository
                    .start(provider.fact_repository.clone());
            } else {
                trace!("Session Ended - Stopping repos...");
                provider.lesson_repository.stop();
                provider.fact_repository.stop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{
            auth::api_mocks::TokenApiMocks,
            facts::api_mocks::FactsApiMocks,
            lessons::api_mocks::{mock_lessons, LessonApiMocks},
        },
        domain::{fake_domain, DomainError},
    };
    use serial_test::serial;
    use std::ops::DerefMut;
    use wg::WaitGroup;

    #[serial]
    #[tokio::test]
    async fn test_login_success() {
        let mut server = mockito::Server::new_async().await;
        server.deref_mut().mock_login_success();
        let mock_lessons = mock_lessons(1);
        server
            .deref_mut()
            .mock_facts_success(mock_lessons[0].id, Vec::new(), 0, true, 1, None);
        server
            .deref_mut()
            .mock_lessons_success(mock_lessons, 0, true, 1, None);

        let domain = fake_domain(server.url() + "/").await.unwrap();

        let r = domain
            .login("user".to_string(), "password".to_string())
            .await;
        assert!(r.is_ok());

        let wg = WaitGroup::new();
        let t_wg = wg.add(1);

        let callback = move |session: Session| {
            trace!("got: {:?}", session);
            t_wg.done();
        };

        let s = domain.get_session(Arc::new(callback)).await;

        assert!(s.is_ok());
        let result = s.unwrap();
        assert_eq!(Session::Authenticated("user".into()), result.session);
    }

    #[serial]
    #[tokio::test]
    async fn test_login_failure() {
        let mut server = mockito::Server::new_async().await;

        let domain = fake_domain(server.url() + "/").await.unwrap();

        server.deref_mut().mock_login_http_failure(401);
        let r1 = domain
            .login("user".to_string(), "password".to_string())
            .await;
        assert!(r1.is_err());
        assert_eq!(
            DomainError::Auth(AuthError::InvalidCredentials),
            r1.unwrap_err()
        );

        server.deref_mut().mock_login_http_failure(500);
        let r2 = domain
            .login("user".to_string(), "password".to_string())
            .await;
        assert!(r2.is_err());
        assert!(matches!(r2, Err(DomainError::Api(_))));

        server.deref_mut().mock_login_other_failure();
        let r3 = domain
            .login("user".to_string(), "password".to_string())
            .await;
        assert!(r3.is_err());
        assert!(matches!(r3, Err(DomainError::Api(_))));
    }
}
