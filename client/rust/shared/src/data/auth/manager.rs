use super::{api::TokenApi, db::TokenDao};
use crate::{
    data::{api::Api, auth::api::TokenApiError, db::Db, SessionManager},
    domain::{
        auth::{AuthError, Session},
        DomainError,
    },
};
use log::{error, trace};
use reqwest::RequestBuilder;
use std::rc::Rc;

impl From<TokenApiError> for DomainError {
    fn from(error: TokenApiError) -> Self {
        match error {
            TokenApiError::Unauthorised() => DomainError::Auth(AuthError::InvalidCredentials),
            TokenApiError::Unexpected(s) => DomainError::Api(s),
        }
    }
}

impl SessionManager {
    pub(in crate::data) fn new(api: Rc<Api>, db: Rc<Db>) -> Self {
        let (tx, rx) = tokio::sync::watch::channel(Session::None);
        let manager = SessionManager {
            state: rx,
            state_mut: tx,
            api: api.clone(),
            db: db.clone(),
        };

        // We're on the runtime thread at construction, so we can seed the initial session
        // synchronously from the database.
        let session = match db.get_token() {
            Ok(token) => token.map_or(Session::None, |token| {
                Session::Authenticated(token.username)
            }),
            Err(e) => {
                error!("Failed to restore session from database: {e:?}");
                Session::None
            }
        };
        trace!("Initial Session from database {:?}", session);
        let _ = manager.state_mut.send(session);

        manager
    }

    pub(crate) async fn login(
        &self,
        username: String,
        password: String,
    ) -> anyhow::Result<Session, DomainError> {
        trace!("session_manager::login()");
        let api = self.api.clone();
        let session = api.login(username.clone(), password).await?;
        trace!("api::Login response {:?}", session);
        let db = self.db.clone();
        db.set_token(username.clone(), session.access, session.refresh)?;
        let session = Session::Authenticated(username);
        trace!("New session: {:?}", session);
        let r = self.state_mut.send(session.clone());
        trace!("rc: {:?}", r);
        anyhow::Result::Ok(session)
    }

    pub(crate) async fn logout(&self) -> uniffi::Result<(), DomainError> {
        let db = self.db.clone();
        db.del_token()?;
        Ok(())
    }

    pub(crate) async fn decorate(&self, builder: RequestBuilder) -> RequestBuilder {
        let db = self.db.clone();
        match db.get_token() {
            Ok(session) => match session {
                Some(token) => {
                    #[cfg(test)]
                    trace!("Using test session token: {}", token.auth_token);
                    builder.bearer_auth(token.auth_token)
                }
                None => builder,
            },
            Err(e) => {
                error!("{:?}", e);
                builder
            }
        }
    }
}
