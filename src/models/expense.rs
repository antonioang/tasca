use time::Date;

#[derive(Debug)]
pub struct Expense {
    pub id: i32,
    pub description: String,
    pub amount: f64,
    pub date: Option<Date>,
}
