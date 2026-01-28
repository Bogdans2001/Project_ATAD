use chrono::NaiveDate;
use crate::core::transaction_service::{
    AddTransactionCommand, AddTransactionError, ReportResult, TransactionService
};
use crate::core::budget_service::*;
use crate::domain::{Transaction,TransactionError, Category};
use crate::persistence::transaction_repository::{TransactionRepository, TransactionSearchFilter};
use crate::persistence::budget_repository::BudgetRepository;
pub struct FinanceApp<TR, BR>
where
    TR: TransactionRepository,
    BR: BudgetRepository,
{
    transaction_service: TransactionService<TR>,
    budget_service: BudgetService<BR>,
}


fn bar(value: f64, max: f64) -> String {
        let width = 30;
        if max <= 0.0 { return " ".repeat(width); }
        let n = ((value / max) * width as f64).round() as usize;
        let empty=width-n;
        format!("{}{}","█".repeat(n), " ".repeat(empty))
    }

impl<TR, BR> FinanceApp<TR, BR>
where
    TR: TransactionRepository,
    BR: BudgetRepository,
{
    pub fn new(transaction_service: TransactionService<TR>, budget_service: BudgetService<BR>) -> Self {
        Self { transaction_service, budget_service, }
    }

    pub fn select(&self) -> anyhow::Result<Vec<Vec<String>>> {
        self.transaction_service.select()
    }


    pub fn add(&self,kind: String, date: NaiveDate, amount: f64, category_id: i64, description: String,) -> Result<(), AddTransactionError> {

        let local_category_id = match category_id {
            0 => Ok(Category::build_category(&description).unwrap_or(1)),
            1..12 => Ok(category_id),
            _ => Err(AddTransactionError::Domain(TransactionError::CategoryIdNotFound)),
            }?;
        
        if kind=="expense" {
            let budget = self.budget_service.select(local_category_id,date, amount)
                .map_err(|_| AddTransactionError::Domain(TransactionError::CategoryIdNotFound))?;
            let mut expenses = self.transaction_service.monthly_expenses(date, local_category_id).
                map_err(|_| AddTransactionError::Domain(TransactionError::CategoryIdNotFound))?;
            
            expenses = expenses + amount;

            if budget==-1.0 {
                println!("Budget not set!");
            }

            if budget<expenses && budget>=0.0 {
                println!("Budget overrun!");
            }
        }

        let cmd = AddTransactionCommand {
            date,
            amount,
            kind: kind,
            category_id: local_category_id,
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

    pub fn add_budget(&self, category_id:i64, month:String, amount:f64) ->anyhow::Result<()>{
        self.budget_service.insert_or_update(category_id, month, amount)?;
        Ok(())
    }

    pub fn import(&self, option:String, path:String) ->anyhow::Result<()>{
        self.transaction_service.import(option, path)?;
        Ok(())
    }

}

