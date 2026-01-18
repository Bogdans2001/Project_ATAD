use chrono::NaiveDate;
use crate::core::transaction_service::{
    AddTransactionCommand, AddTransactionError, ReportResult, TransactionService
};
use crate::domain::Transaction;
use crate::persistence::transaction_repository::{TransactionRepository, TransactionSearchFilter};
pub struct FinanceApp<R: TransactionRepository> {
    transaction_service: TransactionService<R>,
}

fn bar(value: f64, max: f64) -> String {
        let width = 30;
        if max <= 0.0 { return " ".repeat(width); }
        let n = ((value / max) * width as f64).round() as usize;
        let empty=width-n;
        format!("{}{}","█".repeat(n), " ".repeat(empty))
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
        let result = self.transaction_service.report(property)?;
        match result{
                ReportResult::Monthly(rows) => {
                let mut maxx = rows[0].expense;
                for element in &rows{
                    if maxx < element.expense {
                        maxx = element.expense;
                    }
                    if maxx < element.income{
                        maxx = element.income;
                    }
                }
                for element in &rows{
                    println!("{}", element.month);
                    println!("income  {}  {}", bar(element.income, maxx), element.income);
                    println!("expense {}  {}", bar(element.expense, maxx), element.expense);
                    println!();
                }
            }

            ReportResult::Category(rows) => {
                let mut maxx = rows[0].amount;
                for element in &rows{
                    if maxx < element.amount {
                        maxx = element.amount;
                    }
                }
                for element in &rows{
                    println!("{:<20} {} {:>10}", element.name, bar(element.amount, maxx), element.amount);
                }
            }
        }
        Ok(())
    }

}

