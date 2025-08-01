use crate::data::{
    db::Db,
    settings::db::{Setting, SettingDao},
};

#[allow(dead_code)]
pub(crate) struct DbFixtures {}

#[allow(dead_code)]
impl DbFixtures {
    fn create_number(db: &Db, key: String, number: u64) {
        db.put(key.as_str(), Setting::Number(number)).unwrap();
    }
}
