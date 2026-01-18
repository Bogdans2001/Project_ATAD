use chrono::NaiveDate;

use crate::core::category_service::build_category;
use crate::domain::{Transaction, TransactionError, TransactionKind};
use crate::persistence::transaction_repository::{MonthlyReport, TransactionRepository, TransactionSearchFilter};


pub struct AddTransactionCommand {
    pub date: NaiveDate,
    pub amount: f64,
    pub kind: String,
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
        let transaction_kind = match cmd.kind.as_str() {
            "expense" => Ok(TransactionKind::Expense),
            "income" => Ok(TransactionKind::Income),
            _ => Err(AddTransactionError::Domain(TransactionError::ExpenseIncomeNotFound)),
            }?;
        let local_category_id = match cmd.category_id {
            0 => Ok(build_category(&cmd.description).unwrap_or(1)),
            1..12 => Ok(cmd.category_id),
            _ => Err(AddTransactionError::Domain(TransactionError::CategoryIdNotFound)),
            }?;

        let tx = Transaction::new(
            cmd.date,
            cmd.amount,
            transaction_kind,
            local_category_id,
            cmd.description,
        )?;

        self.repo
            .insert(&tx)
            .map_err(AddTransactionError::Persistence)
    }

    pub fn search(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>> {
        self.repo.search(filter)
    }

    pub fn monthly_report(&self, property:String) -> anyhow::Result<Vec<MonthlyReport>> {
        match property.as_str(){
            "monthly" => self.repo.monthly_report(),
            _ => self.repo.monthly_report(),
        }
    }
}

