use crate::data::db::Db;
use rusqlite::{Error, OptionalExtension};

/**
 * Setting
 */
pub(super) struct Token {
    pub(super) username: String,
    auth_token: String,
    refresh_token: String,
}
pub(super) trait TokenDao {
    fn get_token(&self) -> rusqlite::Result<Option<Token>>;
    fn set_token(&self, username: String, auth_token: String, refresh_token: String) -> rusqlite::Result<()>;
    fn del_token(&self) -> rusqlite::Result<()>;
}
impl TokenDao for Db {
    fn get_token(&self) -> rusqlite::Result<Option<Token>> {
        self.connection.query_row(
            "SELECT username, authToken, refreshToken FROM token WHERE id = 1;",
            [],
            |row| Ok(Token { username: row.get(0)?, auth_token: row.get(1)?, refresh_token: row.get(2)? })
        ).optional()
    }

    fn set_token(&self, username: String, auth_token: String, refresh_token: String) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO token(id, username, authToken, refreshToken) VALUES (1, ?, ?, ?);",
            rusqlite::params![username, auth_token, refresh_token]
        )?;
        Ok(())
    }

    fn del_token(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "DELETE FROM token;",
            []
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let db = Db::open("blah.txt".to_string()).unwrap();
        let r = db.get_token();
        assert!(r.unwrap().is_none());
        let r = db.set_token("user1".to_string(), "token1".to_string(), "refresh1".to_string());
        assert!(r.is_ok());
        let r = db.get_token();
        assert!(r.unwrap().is_some_and(|x| x.username == "user1" && x.auth_token == "token1" && x.refresh_token == "refresh1"));
        let r = db.set_token("user2".to_string(), "token2".to_string(), "refresh2".to_string());
        assert!(r.is_ok());
        let r = db.get_token();
        assert!(r.unwrap().is_some_and(|x| x.username == "user2" && x.auth_token == "token2" && x.refresh_token == "refresh2"));
        let r = db.del_token();
        assert!(r.is_ok());
        let r = db.get_token();
        assert!(r.unwrap().is_none());
    }
}
