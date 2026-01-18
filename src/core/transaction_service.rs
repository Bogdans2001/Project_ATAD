use chrono::NaiveDate;

use crate::domain::{Transaction, TransactionError, TransactionKind};
use crate::persistence::transaction_repository::{MonthlyReport, TransactionRepository, TransactionSearchFilter, CategoryReport};


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

pub enum ReportResult {
    Monthly(Vec<MonthlyReport>),
    Category(Vec<CategoryReport>),
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

        let tx = Transaction::new(
            cmd.date,
            cmd.amount,
            transaction_kind,
            cmd.category_id,
            cmd.description,
        )?;

        self.repo
            .insert(&tx)
            .map_err(AddTransactionError::Persistence)
    }

    pub fn search(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>> {
        self.repo.search(filter)
    }
    pub fn monthly_expenses(&self, date:NaiveDate, category_id:i64) -> anyhow::Result<f64> {
        self.repo.monthly_expenses(date,category_id)
    }

    pub fn report(&self, property:String) -> anyhow::Result<ReportResult> {
        let result = match property.as_str(){
            "monthly" => ReportResult::Monthly(self.repo.monthly_report()?),
            _ => ReportResult::Category(self.repo.category_report()?),
        };
        Ok(result)
    }
}

