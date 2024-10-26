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

/// Represents a data, which owns a connection
/// Capabilities are added to the Db via traits per feature
pub(crate) struct Db {
    pub(super) connection: Connection,
}

/// Mapper from a rusqlite error to domain error
impl From<rusqlite::Error> for DomainError {
    #[inline]
    fn from(value: rusqlite::Error) -> Self {
        DomainError::Database(format!("{:?}", value))
    }
}

/// Mapper from a ruslite migration error to a domain error
impl From<rusqlite_migration::Error> for DomainError {
    #[inline]
    fn from(value: rusqlite_migration::Error) -> Self {
        DomainError::Database(value.to_string())
    }
}

/// Basic implementation of the database, with support for opening a connection and initalising or migrating the schema version
impl Db {
    #[cfg(not(test))]
    pub(crate) fn open(path: String) -> std::result::Result<Self, DomainError> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    #[cfg(test)]
    pub(crate) fn open(_path: String) -> std::result::Result<Self, DomainError> {
        // When running tests we are using an in memory db instance
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

/// Generic trait for a model that maps from a rusqlite Row
pub(super) trait RowMapper<T> {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_test() {
        // Test that migrations are working. For now this is migrating from an empty in memory
        // db through to the current latest schema version.
        assert!(MIGRATIONS.validate().is_ok());
    }
}
