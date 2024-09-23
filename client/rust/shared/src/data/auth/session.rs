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

#[cfg(test)]
mod tests {
    use crate::{arc_mutex, data::api::Api, domain::auth::SessionManager};

    use super::*;

    #[tokio::test]
    async fn test_no_session() {
        let manager = SessionManager {
            api: arc_mutex(Api::new("http://127.0.0.1:8000/api/v1/".to_string()).unwrap()),
            db: arc_mutex(Db::open("blah.txt".to_string()).unwrap())
        };
        let session = manager.get_session().await.unwrap();
        assert!(session == Session::None);
    }

    #[tokio::test]
    async fn test_login_and_logout() {
        let manager = SessionManager {
            api: arc_mutex(Api::new("http://127.0.0.1:8000/api/v1/".to_string()).unwrap()),
            db: arc_mutex(Db::open("blah.txt".to_string()).unwrap())
        };
        let result = manager.login("admin".to_string(), "admin".to_string()).await;
        assert!(result.is_ok_and(|session| session == Session::Authenticated("admin".to_string())));

        // Now we should be able to also get a session from the db
        let session = manager.get_session().await.unwrap();
        assert!(session == Session::Authenticated("admin".to_string()));
    }
}
