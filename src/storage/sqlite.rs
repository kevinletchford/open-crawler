use rusqlite::{Connection, Result, params};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pages (
            url TEXT PRIMARY KEY,
            title TEXT,
            content TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_page(conn: &Connection, url: &str, title: &str, content: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO pages (url, title, content) VALUES (?1, ?2, ?3)",
        params![url, title, content],
    )?;
    Ok(())
}
