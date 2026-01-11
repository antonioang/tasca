/*
 * Create tables constants
 * */
pub const CREATE_EXPENSES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    amount REAL NOT NULL,
    date TEXT NOT NULL
);
"#;

/*
 * Query
 * */
pub const SELECT_ALL_EXPENSES: &str = "SELECT id, description, amount, date FROM expenses";
pub const SELECT_EXPENSES_PAGINATED: &str = "SELECT id, description, amount, date FROM expenses LIMIT ?1 OFFSET ?2";

pub const INSERT_EXPENSE: &str = "INSERT INTO expenses (description, amount, date) VALUES (?1, ?2, ?3)";

pub const DELETE_EXPENSE: &str = "DELETE FROM expenses WHERE id = ?1";
