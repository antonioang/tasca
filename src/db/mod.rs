pub mod expense_repo;
mod schema;

use schema::*;

use color_eyre::eyre::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub fn init_db_connection(db_path: &str) -> Result<Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).expect("Failed to create pool");
    
    let mut conn = pool.get().unwrap();
    let trans = conn.transaction()?;

    trans.execute(CREATE_EXPENSES_TABLE, [])?;

    trans.commit()?;
    
    Ok(pool)
}
