use rusqlite::params;

use crate::core::TransactionRepository;
use crate::domain::{Transaction, TransactionKind};

use super::connection_provider::SQLiteConnectionProvider;

pub struct SQLiteTransactionRepository {
    provider: SQLiteConnectionProvider,
}

impl SQLiteTransactionRepository {
    pub fn new(provider: SQLiteConnectionProvider) -> Self {
        Self { provider }
    }
}

impl TransactionRepository for SQLiteTransactionRepository {
    fn insert(&self, tx: &Transaction) -> anyhow::Result<()> {
        let kind = match tx.kind {
            TransactionKind::Income => "income",
            TransactionKind::Expense => "expense",
        };

        self.provider.conn().execute(
            r#"
            INSERT INTO transactions (id, date, amount, kind, category_id, description)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![tx.id.to_string(), tx.date.to_string(),tx.amount, kind, tx.category_id, tx.description,],
        )?;

        Ok(())
    }
}
