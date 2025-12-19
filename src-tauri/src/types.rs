use std::fmt;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(sqlx::FromRow)]
pub struct Transaction {
    id: u64,
    pub name: String,
    pub amount: Decimal,
    pub date: NaiveDate,
    account_id: u64,
    category_id: u64,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction: {}, {}, {}", self.date, self.name, self.amount)
    }
}