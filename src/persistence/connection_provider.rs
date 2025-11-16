use rusqlite::Connection;

pub struct SQLiteConnectionProvider {
    conn: Connection,
}

impl SQLiteConnectionProvider {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                date TEXT NOT NULL,
                amount REAL NOT NULL,
                kind TEXT NOT NULL,
                category_id INTEGER NOT NULL,
                description TEXT NOT NULL
            );
            "#,
        )?;

        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
