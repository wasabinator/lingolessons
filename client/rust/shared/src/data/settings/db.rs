use crate::data::db::Db;
use rusqlite::OptionalExtension;

/**
 * Setting
 */
pub(super) struct Setting {
    key: String,
    value: String,
}
pub(super) trait SettingDao {
    fn get_setting(&self, key: String) -> rusqlite::Result<Option<Setting>>;
    fn set_setting(&self, key: String, value: String) -> rusqlite::Result<()>;
}
impl SettingDao for Db {
    fn get_setting(&self, key: String) -> rusqlite::Result<Option<Setting>> {
        self.connection.query_row(
            "SELECT value FROM setting WHERE key = ?;",
            [key.clone()],
            |row| Ok(Setting { key: key, value: row.get(0)? })
        ).optional()
    }

    fn set_setting(&self, key: String, value: String) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO setting(key, value, modified) VALUES (?, ?, CAST(strftime('%s', 'now') AS INTEGER));",
            rusqlite::params![key, value]
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let db = Db::open("blah.txt".to_string()).unwrap();
        let r = db.get_setting("123".to_string());
        assert!(r.unwrap().is_none());
        let r = db.set_setting("123".to_string(), "abc".to_string());
        assert!(r.is_ok());
        let r = db.get_setting("123".to_string());
        assert!(r.unwrap().is_some_and(|x| x.value == "abc"));
        let r = db.set_setting("123".to_string(), "def".to_string());
        assert!(r.is_ok());
        let r = db.get_setting("123".to_string());
        assert!(r.unwrap().is_some_and(|x| x.value == "def"));
    }
}
