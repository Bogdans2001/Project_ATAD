use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use crate::domain::TransactionKind;
use crate::core::finance_app::FinanceApp;
use crate::core::transaction_service::AddTransactionError;
use crate::persistence::transaction_repository::{TransactionRepository,TransactionSearchFilter};
use crate::persistence::budget_repository::BudgetRepository;
use crate::tui::run_tui;

#[derive(Parser)]
#[command(name = "finance_manager")]
#[command(about = "Personal finance manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long)] kind: String,
        #[arg(long)] amount: f64,
        #[arg(long)] category_id: i64,
        #[arg(long)] date: String,
        #[arg(long)] description: String,
    },
    Search {
        #[arg(long)] description: Option<String>,
        #[arg(long)] kind: Option<String>,
        #[arg(long)] category_id: Option<i64>,
        #[arg(long)] from: Option<String>, 
        #[arg(long)] to: Option<String>,
        #[arg(long)] date: Option<String>,   
        #[arg(long)] limit: Option<i64>,
    },

    Report {
        #[arg(long)] property: String,
    },

    Budget {
        #[arg(long)] category_id: i64,
        #[arg(long)] month: String,
        #[arg(long)] amount: f64,
    },

    Import {
        #[arg(long)] option: String,
        #[arg(long)] path: String,
    },

    Tui {
    }
}

pub fn run<TR, BR>(app: &mut FinanceApp<TR, BR>) -> anyhow::Result<()>
where
    TR: TransactionRepository,
    BR: BudgetRepository,
{
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {kind, amount, category_id, date, description,} => {
            let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
            match app.add(kind, date, amount, category_id, description) {
                Ok(()) => println!("Income|Expense added."),
                Err(e) => match e {
                        AddTransactionError::Domain(domain_err) => {
                        eprintln!("Domain error when adding income|expense: {domain_err:?}");
                    }
                        AddTransactionError::Persistence(persist_err) => {
                        eprintln!("Persistence error when adding income|expense: {persist_err:?}");
                    }
                },
            }
        }

        Commands::Search { description, kind, category_id, from, to, date, limit } => {
            let kind = match kind.as_deref() {
                Some("income") => Some(TransactionKind::Income),
                Some("expense") => Some(TransactionKind::Expense),
                Some(_) => {
                    eprintln!("Invalid --kind. Use income|expense.");
                    return Ok(());
                }
                None => None,
            };

            let from = match from {
                Some(s) => Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")?),
                None => None,
            };

            let to = match to {
                Some(s) => Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")?),
                None => None,
            };

            let date = match date {
                Some(s) => Some(NaiveDate::parse_from_str(&s, "%Y-%m-%d")?),
                None => None,
            };

            let filter = TransactionSearchFilter {
                description,
                from,
                to,
                date,
                kind,
                category_id,
                limit,
            };

            let results = app.search_transactions(filter)?;

            if results.is_empty() {
                println!("No transactions found.");
            } else {
                for t in results {
                    println!(
                        "{} | {} | {} | {} | {}",
                        t.date,
                        match t.kind { TransactionKind::Income => "income", TransactionKind::Expense => "expense" },
                        t.amount,
                        t.category_id,
                        t.description
                    );
                }
            }
        }

        Commands::Report {property} => {
            let property = match property.as_str() {
                "monthly" => "monthly",
                "category" => "category",
                _ => {
                    eprintln!("Invalid --propety. Use monthly|category.");
                    return Ok(());
                }
            };
            app.report(property.to_string())?;

        }

        Commands::Budget {category_id, month, amount} => {
            app.add_budget(category_id,month,amount)?;
            println!("Budget added!");
        }

        Commands::Import {option, path} => {
            app.import(option,path)?;
            println!("Import added!");
        }

        Commands::Tui{} => {
            run_tui(app)?;
        }


    }

    

    Ok(())
}

