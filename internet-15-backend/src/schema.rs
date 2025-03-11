use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FileMetadata {
    pub id: i32,
    pub filename: String,
    pub file_type: String,
    pub file_size: u64,
    pub uploaded_at: u64,
}

pub fn initialize_db() -> Result<Connection> {
    let conn = Connection::open("file_metadata.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            file_type TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            uploaded_at INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
