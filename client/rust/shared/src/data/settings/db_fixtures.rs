use crate::data::db::Db;
use crate::data::settings::db::{Setting, SettingDao};

#[allow(dead_code)]
pub(crate) struct DbFixtures {}

#[allow(dead_code)]
impl DbFixtures {
    fn create_number(db: &Db, key: String, number: u64) {
        let _ = db.put(key.as_str(), Setting::Number(number)).unwrap();
    }
}
