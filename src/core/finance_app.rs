use chrono::NaiveDate;
use crate::core::transaction_service::{
    TransactionService,
    AddTransactionCommand,
    AddTransactionError,
};
use crate::domain::{Transaction,TransactionKind};
use crate::persistence::transaction_repository::{TransactionRepository, TransactionSearchFilter};
pub struct FinanceApp<R: TransactionRepository> {
    transaction_service: TransactionService<R>,
}

impl<R: TransactionRepository> FinanceApp<R> {
    pub fn new(transaction_service: TransactionService<R>) -> Self {
        Self { transaction_service }
    }

    pub fn add_income(
        &self,
        date: NaiveDate,
        amount: f64,
        category_id: i64,
        description: String,
    ) -> Result<(), AddTransactionError> {
        let cmd = AddTransactionCommand {
            date,
            amount,
            kind: TransactionKind::Income,
            category_id,
            description,
        };

        self.transaction_service.add_transaction(cmd)
    }

    pub fn add_expense(
        &self,
        date: NaiveDate,
        amount: f64,
        category_id: i64,
        description: String,
    ) -> Result<(), AddTransactionError> {
        let cmd = AddTransactionCommand {
            date,
            amount,
            kind: TransactionKind::Expense,
            category_id,
            description,
        };

        self.transaction_service.add_transaction(cmd)
    }
}

impl<R: TransactionRepository> FinanceApp<R> {
    pub fn search_transactions(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>> {
        Ok(self.transaction_service.search(filter)?)
    }
}
