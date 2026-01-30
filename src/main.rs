mod domain;
mod core;
mod persistence;
mod cli;
mod importers;
mod tui;

use core::transaction_service::TransactionService;
use core::budget_service::BudgetService;
use core::finance_app::FinanceApp;
use persistence::{SQLiteConnectionProvider, SQLiteTransactionRepository, SQLiteBudgetRepository};

fn main() -> anyhow::Result<()> {
    let transaction_provider = SQLiteConnectionProvider::new("data/finance.db")?;
    let budget_provider = SQLiteConnectionProvider::new("data/finance.db")?;
    let transaction_repository = SQLiteTransactionRepository::new(transaction_provider);
    let budget_repository = SQLiteBudgetRepository::new(budget_provider);
    let transaction_service = TransactionService::new(transaction_repository);
    let budget_service = BudgetService::new(budget_repository);
    let mut app = FinanceApp::new(transaction_service, budget_service);
    cli::run(&mut app)?;
    Ok(())
}
