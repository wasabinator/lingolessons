use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use crate::domain::DomainError;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/data/db/migrations");

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

pub(crate) struct Db {
    pub(super) connection: Connection,
}

impl From<rusqlite::Error> for DomainError {
    #[inline]
    fn from(value: rusqlite::Error) -> Self {
        DomainError::Database(format!("{:?}", value))
    }
}

impl From<rusqlite_migration::Error> for DomainError {
    #[inline]
    fn from(value: rusqlite_migration::Error) -> Self {
        DomainError::Database(value.to_string())
    }
}

impl Db {
    //#[cfg(not(test))]
    pub(crate) fn open(path: String) -> std::result::Result<Self, DomainError> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    #[cfg(test)]
    pub(crate) fn open(_path: String) -> std::result::Result<Self, DomainError> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    fn init(mut conn: Connection) -> std::result::Result<Self, DomainError> {
        match MIGRATIONS.to_latest(&mut conn) {
           Ok(_) => {
                Ok(Self {
                    connection: conn
                })
           },
           Err(err) => {
               let _ = conn.close();
               Err(err.into())
           }
        }
    }
}

// Test that migrations are working
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_test() {
        assert!(MIGRATIONS.validate().is_ok());
    }

    // #[test]
    // fn dm_test() {
    //     let db = Db::open("/Users/amiceli/app.db".to_string());
    //     println!("OK");
    // }
}
