use chrono::NaiveDate;
use anyhow::anyhow;

use crate::domain::{Transaction, TransactionError, TransactionKind,Category};
use crate::persistence::transaction_repository::{MonthlyReport, TransactionRepository, TransactionSearchFilter, CategoryReport};
use crate::importers::{read_csv_as_arrays, read_ofx_as_arrays};


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

    pub fn select(&self) -> anyhow::Result<Vec<Vec<String>>> {
        self.repo.select_transaction()
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

    pub fn import(&self, option:String, path:String) -> anyhow::Result<()> {
        
        let rows = match option.as_str(){
            "csv" =>read_csv_as_arrays(&path),
            "ofx" =>read_ofx_as_arrays(&path),
            _=>Err(anyhow!("Invalid option")),
        }?;
        for row in rows{
            let date = NaiveDate::parse_from_str(&row[0], "%Y-%m-%d").map_err(|_| anyhow!("Invalid date"))?;
            let amount:f64=row[1].parse().map_err(|_| anyhow!("Invalid amount"))?;
            let kind=match row[2].as_str(){
                "expense"=>TransactionKind::Expense,
                "income"=>TransactionKind::Income,
                _=>return Err(anyhow!("Invalid kind")),
            };
            let category_id:i64=row[3].parse().map_err(|_| anyhow!("Invalid category id"))?;
            let description = row[4].clone();

            let local_category_id = match category_id {
                0 => Category::build_category(&description).unwrap_or(1),
                1..12 => category_id,
                _ => return Err(anyhow!("Invalid category"))
            };
            let transaction = Transaction::new( date, amount, kind, local_category_id, description).map_err(|_| anyhow!("Invalid transaction"))?;
            self.repo.insert(&transaction)?;
        }
        Ok(())
    }
}

