pub(crate) mod auth;
pub(crate) mod settings;
pub(crate) mod db;
pub(crate) mod api;

use std::result::Result;

use api::Api;

use crate::domain::auth::SessionManager;
use crate::domain::DomainError;
use crate::data::db::Db;
use crate::{arc_mutex, ArcMutex};

pub(crate) struct DataServiceProvider {
    pub(super) session_manager: ArcMutex<SessionManager>,
}

impl DataServiceProvider {
    pub(crate) fn new(
        base_url: String,
        data_path: String
    ) -> Result<DataServiceProvider, DomainError> {
        let api = arc_mutex(Api::new(base_url)?);
        let db = arc_mutex(Db::open(data_path)?);
        let sm = SessionManager::new(api.clone(), db.clone());
        Ok(Self {
            session_manager: arc_mutex(sm),
        })
    }
}
