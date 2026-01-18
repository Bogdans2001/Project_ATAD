use rusqlite::params;
use crate::domain::Budget;
use super::connection_provider::SQLiteConnectionProvider;

pub trait BudgetRepository {
    fn insert(&self, budget: &Budget) -> anyhow::Result<()>;
    fn update(&self, budget: &Budget) -> anyhow::Result<()>;
    fn select(&self, budget: &Budget) -> anyhow::Result<f64>;
    fn select_count(&self, budget: &Budget) -> anyhow::Result<i64>;
}

pub struct SQLiteBudgetRepository {
    provider: SQLiteConnectionProvider,
}


impl SQLiteBudgetRepository {
    pub fn new(provider: SQLiteConnectionProvider) -> Self {
        Self { provider }
    }
}

impl BudgetRepository for SQLiteBudgetRepository{
    fn insert(&self, budget: &Budget) -> anyhow::Result<()> {
        self.provider.conn().execute(
            "INSERT INTO budgets (category_id, month, amount) VALUES (?1, ?2, ?3)",
            params![budget.category_id, budget.month, budget.amount],
        )?;
        Ok(())
    }
    fn update(&self, budget: &Budget) -> anyhow::Result<()> {
        self.provider.conn().execute(
            "UPDATE budgets SET amount=?1 WHERE category_id=?2 AND month=?3;",
            params![budget.amount, budget.category_id, budget.month],
        )?;
        Ok(())
    }
    fn select(&self, budget: &Budget) -> anyhow::Result<f64> {

        let count = self.select_count(budget)?;

        if count == 0{
            return Ok(-1.0);
        }
        let ammount = self.provider.conn().query_row(
            "SELECT amount FROM budgets WHERE category_id=?1 AND month=?2;",
            params![budget.category_id, budget.month],
            |row| row.get(0),
        )?;
        Ok(ammount)
    }

    fn select_count(&self, budget: &Budget) -> anyhow::Result<i64> {
        let count = self.provider.conn().query_row(
            "SELECT COUNT(*) FROM budgets WHERE category_id=?1 AND month=?2;",
            params![budget.category_id, budget.month],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
