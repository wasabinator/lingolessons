use crate::data::db::Db;
use rusqlite::OptionalExtension;

/**
 * Setting
 */
#[derive(Debug, PartialEq)]
pub(super) enum Setting {
    Text(String),
    Number(u64),
    None,
}

impl TryFrom<&rusqlite::Row<'_>> for Setting {
    type Error = rusqlite::Error;
    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<Setting> {
        let text: Option<String> = row.get(0)?;
        let number: Option<u64> = row.get(1)?;

        Ok(if let Some(number) = number {
            Setting::Number(number)
        } else if let Some(str) = text {
            Setting::Text(str)
        } else {
            log::error!("Illegal setting type, passing empty");
            Setting::None
        })
    }
}

#[allow(dead_code)]
pub(super) trait SettingDao {
    fn get(&self, key: &str) -> rusqlite::Result<Setting>;
    fn put(&self, key: &str, value: Setting) -> rusqlite::Result<()>;
}

impl SettingDao for Db {
    fn get(&self, key: &str) -> rusqlite::Result<Setting> {
        self.perform(|conn| {
            conn.query_row(
                "SELECT text, number FROM setting WHERE key = ?;",
                [key],
                |row| Setting::try_from(row),
            )
        })
        .optional()
        .map_or(Ok(Setting::None), |s| Ok(s.unwrap_or(Setting::None)))
    }

    fn put(&self, key: &str, value: Setting) -> rusqlite::Result<()> {
        let params = match value {
            Setting::Text(text) => {
                rusqlite::params![key, text.to_owned(), Option::<u64>::None]
            }
            Setting::Number(number) => {
                rusqlite::params![key, Option::<String>::None, number.to_owned()]
            }
            _ => {
                rusqlite::params![key, Option::<String>::None, Option::<u64>::None]
            }
        };

        self.perform(|conn| { conn.execute(
            "INSERT OR REPLACE INTO setting(key, text, number, modified) VALUES (?, ?, ?, CAST(strftime('%s', 'now') AS INTEGER));",
            params,
        )})?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let db = Db::open("blah.txt".into()).unwrap();
        let r = db.get("123");
        assert_eq!(Ok(Setting::None), r);
        let r = db.put("123", Setting::Text("abc".into()));
        assert!(r.is_ok());
        let r = db.get("123");
        assert_eq!(Ok(Setting::Text("abc".into())), r);
        let r = db.put("123", Setting::Text("def".into()));
        assert!(r.is_ok());
        let r = db.get("123");
        assert_eq!(Ok(Setting::Text("def".into())), r);
    }
}
