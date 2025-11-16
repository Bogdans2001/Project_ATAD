use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::core::finance_app::FinanceApp;
use crate::core::transaction_service::TransactionRepository;
use crate::core::transaction_service::AddTransactionError;

#[derive(Parser)]
#[command(name = "pfm")]
#[command(about = "Personal finance manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddIncome {
        #[arg(long)] amount: f64,
        #[arg(long)] category_id: i64,
        #[arg(long)] date: String,
        #[arg(long)] description: String,
    },
    AddExpense {
        #[arg(long)] amount: f64,
        #[arg(long)] category_id: i64,
        #[arg(long)] date: String,
        #[arg(long)] description: String,
    },
}

pub fn run<R>(app: FinanceApp<R>) -> anyhow::Result<()>
where
    R: TransactionRepository,
{
    let cli = Cli::parse();

    match cli.command {
        Commands::AddIncome {
            amount,
            category_id,
            date,
            description,
        } => {
            let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
            match app.add_income(date, amount, category_id, description) {
                Ok(()) => println!("Income added."),
                Err(e) => match e {
                    AddTransactionError::Domain(domain_err) => {
                        eprintln!("Domain error when adding income: {domain_err:?}");
                    }
                    AddTransactionError::Persistence(persist_err) => {
                        eprintln!("Persistence error when adding income: {persist_err:?}");
                    }
                },
            }
        }
        Commands::AddExpense {
            amount,
            category_id,
            date,
            description,
        } => {
            let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;

            match app.add_expense(date, amount, category_id, description) {
                Ok(()) => println!("Expense added."),
                Err(e) => eprintln!("Failed to add expense: {e:?}"),
            }
        }
    }

    Ok(())
}
