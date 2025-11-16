use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum TransactionKind {
    Income,
    Expense,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub date: NaiveDate,
    pub amount: f64,
    pub kind: TransactionKind,
    pub category_id: i64,
    pub description: String,
}

#[derive(Debug)]
pub enum TransactionError {
    AmountMustBePositive,
    EmptyDescription,
}

impl Transaction {
    pub fn new(
        date: NaiveDate,
        amount: f64,
        kind: TransactionKind,
        category_id: i64,
        description: String,
    ) -> Result<Self, TransactionError> {
        if amount <= 0.0 {
            return Err(TransactionError::AmountMustBePositive);
        }
        if description.trim().is_empty() {
            return Err(TransactionError::EmptyDescription);
        }

        Ok(Transaction {
            id: Uuid::new_v4(),
            date,
            amount,
            kind,
            category_id,
            description,
        })
    }
}
