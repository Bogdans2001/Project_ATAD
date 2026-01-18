

use chrono::NaiveDate;

use crate::domain::{Budget, BudgetError};
use crate::persistence::budget_repository::{BudgetRepository};

pub struct BudgetService<R: BudgetRepository> {
    repo: R,
}

impl<R: BudgetRepository> BudgetService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn insert_or_update(&self, category_id:i64, month:String, amount:f64) -> Result<(), BudgetError> {

        if amount<0.0 {
            return Err(BudgetError::AmountMustBePositive);
        }
        let budget = Budget::new(
            category_id,
            month,
            amount,
        )?;

        let count = self.repo.select_count(&budget).map_err(|_| BudgetError::NoValidRows)?;

        if count==0 {
            self.repo.insert(&budget).map_err(|_| BudgetError::CategoryIdNotFound)?;
        }
        else {
            self.repo.update(&budget).map_err(|_| BudgetError::CategoryIdNotFound)?;
        }
        Ok(())
    }

    pub fn select(&self, category_id:i64, date:NaiveDate, amount:f64) -> Result<f64,BudgetError>{
        let month = date.format("%Y-%m").to_string();
        let budget = Budget::new(
            category_id,
            month,
            amount,
        )?;
        let result = self.repo.select(&budget).map_err(|_| BudgetError::NoValidRows)?;
        Ok(result)
    }


}