use color_eyre::eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use time::{
    Date, 
    macros::{format_description},
};

use crate::{
    db::schema::{DELETE_EXPENSE, INSERT_EXPENSE, SELECT_ALL_EXPENSES, SELECT_EXPENSES_PAGINATED},
    models::expense::Expense,
};

pub struct ExpenseRepo {
    pool: Pool<SqliteConnectionManager>,
}

impl ExpenseRepo {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }
    
    pub fn get(&self, page: i64) -> Result<Vec<Expense>> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(SELECT_EXPENSES_PAGINATED)?;
        let expenses = stmt.query_map([10, page], |r| {
            let date: Option<Date> = r
                .get::<_, Option<String>>(3)?
                .and_then(|s| Date::parse(&s, format_description!("[year]-[month]-[day]")).ok());

            Ok(Expense {
                id: r.get(0)?,
                description: r.get(1)?,
                amount: r.get(2)?,
                date: date
            })
        })?
        .collect::<Result<_, _>>()?;

        Ok(expenses)
    }

    pub fn insert(&self, description: &str, amount: &str, date: &str) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(INSERT_EXPENSE, [description, amount, date])?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(DELETE_EXPENSE, [id])?;
        Ok(())
    }

    pub fn log(&self) -> Result<()> {
        let conn = self.pool.get()?;

        let mut stmt = conn.prepare(SELECT_ALL_EXPENSES)?;
        let expense_iter = stmt.query_map([], |row| {
            let date: Option<Date> = row
                .get::<_, Option<String>>(3)?
                .and_then(|s| Date::parse(&s, format_description!("[year]-[month]-[day]")).ok());
            
            Ok(Expense {
                id: row.get(0)?,
                description: row.get(1)?,
                amount: row.get(2)?,
                date: date,
            })
        })?;

        for expense in expense_iter {
            println!("Found expense {:?}", expense?);
        }
        Ok(())
    }
}
