//! Import a sample domestic transaction.
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example import
//!
//! WARNING: This will submit a real payment order that requires
//! authorization in internet banking before it is processed.

use std::env;

use fiocz_rs::types::transaction::{DomesticTransaction, Import};
use fiocz_rs::Fio;
use rust_decimal::Decimal;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let fio = Fio::new(&token);

    let transaction = DomesticTransaction {
        account_from: env::var("ACCOUNT_FROM").unwrap_or_else(|_| "0000000000".to_string()),
        currency: "CZK".to_string(),
        amount: Decimal::new(100, 2),
        account_to: env::var("ACCOUNT_TO").unwrap_or_else(|_| "0000000000".to_string()),
        bank_code: "2010".to_string(),
        ks: None,
        vs: Some("1234567890".to_string()),
        ss: None,
        date: "2025-12-31".to_string(),
        message_for_recipient: Some("Test payment from fiocz-rs".to_string()),
        comment: None,
        payment_reason: None,
        payment_type: None,
    };

    let import = Import::builder().domestic(transaction).build();

    println!("Generated XML:\n{}\n", import.to_xml());

    match fio.import_transactions(import).await {
        Ok(response) => println!("Import response:\n{response}"),
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
