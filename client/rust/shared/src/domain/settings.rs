use crate::data::db::Db;
use std::rc::Rc;

pub(crate) struct SettingRepository {
    pub(crate) db: Rc<Db>,
}
