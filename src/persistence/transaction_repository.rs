use rusqlite::{params, Error as SqlError, params_from_iter, ToSql};
use chrono::NaiveDate;
use crate::domain::{Transaction, TransactionKind};
use super::connection_provider::SQLiteConnectionProvider;

#[derive(Debug)]
pub struct MonthlyReport {
    pub month: String, 
    pub income: f64,
    pub expense: f64,
}

pub struct CategoryReport{
    pub name: String,
    pub amount: f64,
}


pub trait TransactionRepository {
    fn insert(&self, tx: &Transaction) -> anyhow::Result<()>;
    fn select_transaction(&self) -> anyhow::Result<Vec<Vec<String>>>;
    fn search(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>>;
    fn monthly_report(&self) -> anyhow::Result<Vec<MonthlyReport>>;
    fn category_report(&self) -> anyhow::Result<Vec<CategoryReport>>;
    fn monthly_expenses(&self, date:NaiveDate, category_id:i64) -> anyhow::Result<f64>;
}

pub struct TransactionSearchFilter {
    pub description: Option<String>,
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
    pub date: Option<NaiveDate>,
    pub kind: Option<TransactionKind>,
    pub category_id: Option<i64>,
    pub limit: Option<i64>,
}

pub struct SQLiteTransactionRepository {
    provider: SQLiteConnectionProvider,
}

impl SQLiteTransactionRepository {
    pub fn new(provider: SQLiteConnectionProvider) -> Self {
        Self { provider }
    }
}

impl TransactionRepository for SQLiteTransactionRepository {
    fn insert(&self, tx: &Transaction) -> anyhow::Result<()> {
        let kind = match tx.kind {TransactionKind::Income => "income",TransactionKind::Expense => "expense",};

        self.provider.conn().execute(
            "INSERT INTO transactions (id, date, amount, kind, category_id, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![tx.id.to_string(), tx.date.to_string(),tx.amount, kind, tx.category_id, tx.description,],
        )?;
        Ok(())
    }

    fn select_transaction(&self) -> anyhow::Result<Vec<Vec<String>>>{
        let sql = String::from("SELECT date, amount, kind, category_id, description FROM transactions;");
        let conn = self.provider.conn();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| {
            let mut sql_row = Vec::new();
            let date: String = row.get(0)?;
            let amount: f64 = row.get(1)?;
            let kind: String = row.get(2)?;
            let category_id: i64 = row.get(3)?;
            let description: String = row.get(4)?;

            let amount_str:String = amount.to_string();
            let category_id_str:String = category_id.to_string();

            sql_row.push(date);
            sql_row.push(amount_str);
            sql_row.push(kind);
            sql_row.push(category_id_str);
            sql_row.push(description);

            Ok(sql_row)
        })?;
        
        let sql_rows:Vec<Vec<String>> = rows.filter_map(Result::ok).collect();
        Ok(sql_rows)

    }

    fn search(&self, filter: TransactionSearchFilter) -> anyhow::Result<Vec<Transaction>> {
        let mut sql = String::from("SELECT id, date, amount, kind, category_id, description FROM transactions WHERE 1=1");
        let mut p: Vec<Box<dyn ToSql>> = vec![];
        if let Some(description) = &filter.description {
            sql.push_str(" AND description LIKE ?");
            p.push(Box::new(format!("%{}%", description)));
        }
        if let Some(kind) = &filter.kind {
            sql.push_str(" AND kind = ?");
            let k = match kind {
                TransactionKind::Income => "income",
                TransactionKind::Expense => "expense",
            };
            p.push(Box::new(k.to_string()));
        }
        if let Some(category_id) = filter.category_id {
            sql.push_str(" AND category_id = ?");
            p.push(Box::new(category_id));
        }
        if let Some(from) = filter.from {
            sql.push_str(" AND date >= ?");
            p.push(Box::new(from.to_string()));
        }
        if let Some(to) = filter.to {
            sql.push_str(" AND date <= ?");
            p.push(Box::new(to.to_string()));
        }
        if let Some(date) = filter.date {
            sql.push_str(" AND date = ?");
            p.push(Box::new(date.to_string()));
        }
        sql.push_str(" ORDER BY date DESC");
        if let Some(limit) = filter.limit {
            sql.push_str(" LIMIT ?");
            p.push(Box::new(limit));
        }
        let conn = self.provider.conn();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_from_iter(p.iter().map(|x| x.as_ref())), |row| {
            let id: String = row.get(0)?;
            let date_str: String = row.get(1)?;
            let amount: f64 = row.get(2)?;
            let kind_str: String = row.get(3)?;
            let category_id: i64 = row.get(4)?;
            let description: String = row.get(5)?;
            let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| {
                SqlError::FromSqlConversionFailure(
                    1,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

            let kind = match kind_str.as_str() {
                "income" => TransactionKind::Income,
                "expense" => TransactionKind::Expense,
                other => {
                    return Err(SqlError::FromSqlConversionFailure(
                        3,
                        rusqlite::types::Type::Text,
                        format!("Unknown kind in DB: {}", other).into(),
                    ));
                }
            };

            Ok(Transaction::from_db(id, date, amount, kind, category_id, description))
        }
        )?;
        let mut out = vec![];
        for r in rows {
                out.push(r?);
        }
        Ok(out)
    }

    fn monthly_report(&self) -> anyhow::Result<Vec<MonthlyReport>>{
        let conn = self.provider.conn();
        let mut stmt = conn.prepare("SELECT substr(date, 1, 7) AS month,
            SUM(CASE WHEN kind = 'income' THEN amount ELSE 0 END) AS income, SUM(CASE WHEN kind = 'expense' THEN amount ELSE 0 END) AS expense
            FROM transactions GROUP BY month ORDER BY month",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(MonthlyReport {
                month: row.get(0)?,
                income: row.get(1)?,
                expense: row.get(2)?,
            })
        })?;

        let mut out = vec![];
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }
    fn category_report(&self) -> anyhow::Result<Vec<CategoryReport>>{
        let conn = self.provider.conn();
        let mut stmt = conn.prepare("SELECT c.name AS category_name, SUM(t.amount) AS total_amount FROM transactions t
            JOIN categories c ON c.id = t.category_id GROUP BY c.id;",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(CategoryReport {
                name: row.get(0)?,
                amount: row.get(1)?,
            })
        })?;

        let mut out = vec![];
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    fn monthly_expenses(&self, date:NaiveDate, category_id:i64) -> anyhow::Result<f64> {
        let month = date.format("%Y-%m").to_string();
        let date1 = format!("{}-00",month);
        let date2=  format!("{}-32",month);
        let count = self.provider.conn().query_row("SELECT count(*) FROM transactions WHERE kind='expense' AND date>?1 AND date<?2 AND category_id=?3;",
            params![date1, date2, category_id],
            |row| row.get::<_,i64>(0),
        )?;

        if count == 0 {
            return Ok(0.0);
        }

        let expenses = self.provider.conn().query_row("SELECT SUM(CASE WHEN kind = 'expense' THEN amount ELSE 0 END) AS expense FROM transactions 
        WHERE date>?1 AND date<?2 AND category_id=?3;",
            params![date1, date2, category_id],
            |row| row.get(0),
        )?;
        Ok(expenses)
    }

}
