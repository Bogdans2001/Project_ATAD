use chrono::NaiveDate;
use crate::core::transaction_service::{
    TransactionService,
    AddTransactionCommand,
    AddTransactionError,
};
use crate::domain::Transaction;
use crate::persistence::transaction_repository::{TransactionRepository, TransactionSearchFilter};
pub struct FinanceApp<R: TransactionRepository> {
    transaction_service: TransactionService<R>,
}

fn bar(value: f64, max: f64) -> String {
        let width = 30;
        if max <= 0.0 { return "".into(); }
        let n = ((value / max) * width as f64).round() as usize;
        "█".repeat(n)
    }

impl<R: TransactionRepository> FinanceApp<R> {
    pub fn new(transaction_service: TransactionService<R>) -> Self {
        Self { transaction_service }
    }

    pub fn add(&self,kind: String, date: NaiveDate, amount: f64, category_id: i64, description: String,) -> Result<(), AddTransactionError> {
        let cmd = AddTransactionCommand {
            date,
            amount,
            kind: kind,
            category_id,
            description,
        };

        self.transaction_service.add_transaction(cmd)
    }

    pub fn search_transactions(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>> {
        Ok(self.transaction_service.search(filter)?)
    }


    pub fn report(&self, property: String) -> anyhow::Result<()> {
        let result = self.transaction_service.monthly_report(property)?;
        let mut maxx = result[0].expense;
        for element in &result{
            if maxx < element.expense {
                maxx = element.expense;
            }
            if maxx < element.income{
                maxx = element.income;
            }
        }
        for element in &result{
            println!("{}", element.month);
            println!("income  {}  {}", bar(element.income, maxx), element.income);
            println!("expense {}  {}", bar(element.expense, maxx), element.expense);
            println!();
        }
        
        Ok(())
    }


}

