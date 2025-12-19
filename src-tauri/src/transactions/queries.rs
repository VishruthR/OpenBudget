use sqlx::{Pool, Sqlite};
use crate::types::Transaction;

#[tokio::main]
pub async fn get_transactions(pool: &Pool<Sqlite>, limit: Option<u32>) -> Result<Vec<Transaction>, sqlx::Error> {
    let mut query = "SELECT * FROM transaction ORDER BY date".to_owned();
    if let Some(lim) = limit {
        query.push_str(" LIMIT '%' || $1 || '%'");
    }

    let res: Vec<Transaction> = sqlx::query_as(query.as_str())
        .bind(lim)
        .fetch_all(pool).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_transactions() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}