use reqwest::RequestBuilder;

use crate::{data::{api::Api, db::Db}, domain::{auth::SessionManager, DomainError}, ArcMutex};
use crate::domain::auth::Session;

use super::db::TokenDao;

impl SessionManager {
    pub(in crate::data) async fn new(api: ArcMutex<Api>, db: ArcMutex<Db>) -> Self {
        let api = api.clone();
        let db = db.clone();
        let state = match db.lock().await.get_token() { 
            Ok(token) => token.map_or(Session::None, |token| Session::Authenticated(token.username)),
            Err(_) => Session::None,
        };

        SessionManager {
            state: tokio::sync::watch::Sender::new(state),
            api: api.clone(),
            db: db.clone()
        }
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

    pub(crate) async fn decorate(&self, builder: RequestBuilder) -> RequestBuilder {
        let db = self.db.lock().await;
        match db.get_token() {
            Ok(session) => match session {
                Some(token) => {
                    println!("***** FOUND TOKEN ******{}", token.auth_token);
                    builder.bearer_auth(token.auth_token)
                },
                None => builder
            },
            Err(e) => {
                print!("{:?}", e);
                builder
            }
        }
    }
}
