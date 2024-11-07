use crate::{data::db::Db, ArcMutex};

pub(crate) struct SettingRepository {
    pub(crate) db: ArcMutex<Db>,
}
