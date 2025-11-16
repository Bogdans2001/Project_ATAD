use chrono::NaiveDate;

use crate::domain::{Transaction, TransactionError, TransactionKind};
pub trait TransactionRepository {
    fn insert(&self, tx: &Transaction) -> anyhow::Result<()>;
}

pub struct AddTransactionCommand {
    pub date: NaiveDate,
    pub amount: f64,
    pub kind: TransactionKind,
    pub category_id: i64,
    pub description: String,
}

#[derive(Debug)]
pub enum AddTransactionError {
    Domain(TransactionError),
    Persistence(anyhow::Error),
}

impl From<TransactionError> for AddTransactionError {
    fn from(e: TransactionError) -> Self {
        AddTransactionError::Domain(e)
    }
}

pub struct TransactionService<R: TransactionRepository> {
    repo: R,
}

impl<R: TransactionRepository> TransactionService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn add_transaction(&self, cmd: AddTransactionCommand) -> Result<(), AddTransactionError> {
        let tx = Transaction::new(
            cmd.date,
            cmd.amount,
            cmd.kind,
            cmd.category_id,
            cmd.description,
        )?;

        self.repo
            .insert(&tx)
            .map_err(AddTransactionError::Persistence)
    }
}
