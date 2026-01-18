use thiserror::Error;
pub struct Budget {
    pub category_id:i64,
    pub month: String,
    pub amount: f64,
}

#[derive(Debug, Error)]
pub enum BudgetError {
    #[error("Amount must be positive")]
    AmountMustBePositive,

    #[error("No valid rows")]
    NoValidRows,

    #[error("Category id not found")]
    CategoryIdNotFound,

    #[error(transparent)]
    Db(#[from] anyhow::Error),
}

impl Budget {
    pub fn new(
        category_id:i64,
        month: String,
        amount: f64,
    ) -> Result<Self, BudgetError> {
        if amount <= 0.0 {
            return Err(BudgetError::AmountMustBePositive);
        }
        Ok(Budget {
            category_id,
            month,
            amount,
        })
    }

}