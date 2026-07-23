//! Common SQLite database utilities for companion applications.
//!
//! Provides functions to establish connection parameters, enable WAL mode,
//! and configure connection properties for optimal desktop/server runtime performance.

use rusqlite::{Connection, Result};
use std::path::Path;

/// Establishes an SQLite database connection at the specified file path,
/// enforcing foreign key constraints and enabling Write-Ahead Logging (WAL) mode
/// for concurrent read/write optimization.
pub fn establish_connection<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Enable Write-Ahead Logging (WAL) for better concurrent performance
    conn.pragma_update(None, "journal_mode", "WAL")?;

    // Enforce relational foreign key constraints
    conn.pragma_update(None, "foreign_keys", "ON")?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn establishes_connection_and_enables_wal() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("test.db");
        let conn = establish_connection(&path).expect("connect");

        let journal_mode: String = conn
            .pragma_query_value(None, "journal_mode", |r| r.get(0))
            .expect("journal_mode");
        assert_eq!(journal_mode.to_lowercase(), "wal");

        let fk: i64 = conn
            .pragma_query_value(None, "foreign_keys", |r| r.get(0))
            .expect("foreign_keys");
        assert_eq!(fk, 1);
    }

    #[test]
    fn connection_succeeds_for_in_memory() {
        let conn = establish_connection(":memory:").expect("memory connect");
        let fk: i64 = conn
            .pragma_query_value(None, "foreign_keys", |r| r.get(0))
            .expect("foreign_keys");
        assert_eq!(fk, 1);
    }
}
