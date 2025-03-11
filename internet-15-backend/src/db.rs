use rusqlite::{params, Connection, Result};
use crate::models::FileMetadata;

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

pub fn list_files() -> Result<Vec<FileMetadata>> {
    let conn = initialize_db()?;
    let mut stmt = conn.prepare("SELECT id, filename, file_type, file_size, uploaded_at FROM files")?;
    
    let file_iter = stmt.query_map([], |row| {
        Ok(FileMetadata {
            id: row.get(0)?,
            filename: row.get(1)?,
            file_type: row.get(2)?,
            file_size: row.get(3)?,
            uploaded_at: row.get(4)?,
        })
    })?;

    let files: Vec<FileMetadata> = file_iter.map(|file| file.unwrap()).collect();
    Ok(files)
}
