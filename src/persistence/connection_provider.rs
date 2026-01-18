use rusqlite::Connection;

pub struct SQLiteConnectionProvider {
    conn: Connection,
}

impl SQLiteConnectionProvider {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("CREATE TABLE IF NOT EXISTS transactions (id TEXT PRIMARY KEY, date TEXT NOT NULL, amount REAL NOT NULL, kind TEXT NOT NULL, 
            category_id INTEGER NOT NULL, description TEXT NOT NULL, FOREIGN KEY(category_id) REFERENCES categories(id));",)?;

        conn.execute_batch("CREATE TABLE IF NOT EXISTS categories (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE);",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (1, 'unknown');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (2, 'salary');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (3, 'groceries');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (4, 'transport');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (5, 'furniture');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (6, 'subscriptions');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (7, 'bills');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (8, 'scholarship');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (9, 'business');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (10, 'pension');",)?;
        conn.execute_batch("INSERT OR IGNORE INTO categories (id, name) VALUES (11, 'other income');",)?;
        conn.execute_batch("CREATE TABLE IF NOT EXISTS budgets (id INTEGER PRIMARY KEY AUTOINCREMENT, category_id INTEGER NOT NULL, month TEXT NULL, amount REAL NOT NULL,
            UNIQUE(category_id, month), FOREIGN KEY(category_id) REFERENCES categories(id));"
        ,)?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
