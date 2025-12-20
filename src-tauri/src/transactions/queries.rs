use sqlx::{Pool, Sqlite};
use crate::types::{Transaction, Cents};
use chrono::NaiveDate;
use rust_decimal::dec;

pub async fn get_transactions(pool: &Pool<Sqlite>, limit: Option<u32>) -> Result<Vec<Transaction>, sqlx::Error> {
    // TODO: Look into supporting limit using prepared statements, maybe have to do in separate function
    let mut query = "SELECT * FROM 'transaction' ORDER BY date, id".to_owned();

    let res: Vec<Transaction> = sqlx::query_as(query.as_str())
        .fetch_all(pool).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(fixtures(path="../fixtures", scripts("transactions")))]
    async fn test_get_all_transactions(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>> {
        let transactions_expected = vec![
            Transaction::new(
                1,
                "TRANSACTION 1".to_owned(),
                Cents(dec!(-5.77)),
                NaiveDate::from_ymd_opt(2025, 12, 15).unwrap(),
                1,
                1
            ),
            Transaction::new(
                2,
                "TRANSACTION 2".to_owned(),
                Cents(dec!(-10.90)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1
            ),
            Transaction::new(
                4,
                "TRANSACTION 4".to_owned(),
                Cents(dec!(-0.70)),
                NaiveDate::from_ymd_opt(2025, 12, 16).unwrap(),
                1,
                1
            ),
            Transaction::new(
                3,
                "TRANSACTION 3".to_owned(),
                Cents(dec!(-1.90)),
                NaiveDate::from_ymd_opt(2025, 12, 17).unwrap(),
                1,
                1
            ),
        ];

        let transactions = get_transactions(&pool, Some(0)).await?;
        
        assert_eq!(transactions, transactions_expected);
        Ok(())
    }
}