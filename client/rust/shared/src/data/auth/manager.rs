use crate::{data::{api::Api, db::Db}, domain::{auth::SessionManager, DomainError}, ArcMutex};
use crate::domain::auth::Session;

use super::db::TokenDao;

impl SessionManager {
    pub(in crate::data) fn new(api: ArcMutex<Api>, db: ArcMutex<Db>) -> Self {
        SessionManager {
            api: api.clone(),
            db: db.clone()
        }
    }

    pub(crate) async fn get_session(&self) -> uniffi::Result<Session, DomainError> {
        use super::db::TokenDao;

        let db = self.db.lock().await;
        let state = db.get_token()?;
        let session = state.map_or(Session::None, |token| Session::Authenticated(token.username));
        Ok(session)
    }

    pub(crate) async fn login(&self, username: String, password: String) -> uniffi::Result<Session, DomainError> {
        use super::api::TokenApi;
        use super::db::TokenDao;

        let api = self.api.lock().await;
        let session = api.login(username.clone(), password).await?;
        let db = self.db.lock().await;
        db.set_token(username.clone(), session.access, session.refresh)?;
        uniffi::Result::Ok(Session::Authenticated(username))
    }

    pub(crate) async fn logout(&self) -> uniffi::Result<(), DomainError> {
        let db = self.db.lock().await;
        db.del_token()?;
        Ok(())
    }
}
