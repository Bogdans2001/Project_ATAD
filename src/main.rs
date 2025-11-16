mod domain;
mod core;
mod persistence;
mod cli;

use core::transaction_service::TransactionService;
use core::finance_app::FinanceApp;
use persistence::{SQLiteConnectionProvider, SQLiteTransactionRepository};

fn main() -> anyhow::Result<()> {
    let provider = SQLiteConnectionProvider::new("data/finance.db")?;
    let tx_repo = SQLiteTransactionRepository::new(provider);
    let tx_service = TransactionService::new(tx_repo);
    let app = FinanceApp::new(tx_service);
    cli::run(app)
}
