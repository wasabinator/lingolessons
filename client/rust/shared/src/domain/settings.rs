use crate::data::db::Db;
use std::sync::Arc;

pub(crate) struct SettingRepository {
    pub(crate) db: Arc<Db>,
}
