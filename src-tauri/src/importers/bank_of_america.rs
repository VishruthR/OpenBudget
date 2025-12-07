use std::fs::File;
use std::io::{prelude::*, BufReader};
use crate::importers::types::Transaction;

const TRANSACTIONS_PATH: &str = "../data/bankofamericatransactions.txt";

pub fn load_file(filename: &String) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    return Ok(reader);
}

// TODO: Make this function more idiomatic, maybe less brittle
pub fn parse_statement(filename: &String) -> Result<Vec<Transaction>, std::io::Error> {
    let reader = load_file(filename)?;
    let mut transactions: Vec<Transaction> = Vec::new();

    let mut found_header = false;
    for line in reader.lines() {
        // Find header row for transactions
        let transaction_line = line?;
        if transaction_line.starts_with("Date") {
            found_header = true;
            continue;
        }

        if found_header {
            let transaction_parts: Vec<&str> = transaction_line.split("  ").filter(|part| *part != "").collect();
            let transaction = Transaction {
                date: transaction_parts[0].to_string(),
                description: transaction_parts[1].to_string(),
                amount: transaction_parts[2].to_string(),
            };

            transactions.push(transaction);
        }
    }

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_statement() -> Result<(), std::io::Error> {
        let transactions= parse_statement(&TRANSACTIONS_PATH.to_string())?;

        for transaction in transactions.iter() {
            println!("{}", transaction);
        }
        
        Ok(())
    }
}