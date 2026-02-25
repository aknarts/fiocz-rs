//! Fetch account movements since last download.
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example movements_since_last [format]

use std::env;

use fiocz_rs::types::ExportFormat;
use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let fio = Fio::new(&token);
    let args: Vec<String> = env::args().collect();

    if let Some(fmt) = args.get(1) {
        let format = match fmt.as_str() {
            "json" => ExportFormat::Json,
            "xml" => ExportFormat::Xml,
            "csv" => ExportFormat::Csv,
            other => {
                eprintln!("Unknown format: {other}");
                std::process::exit(1);
            }
        };
        match fio.movements_since_last_raw(format).await {
            Ok(body) => println!("{body}"),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    } else {
        match fio.movements_since_last().await {
            Ok(stmt) => {
                let txns = &stmt.account_statement.transaction_list.transaction;
                println!("Transactions since last download: {}", txns.len());
                for (i, txn) in txns.iter().enumerate() {
                    if let Some(Some(id)) = txn.get("column22") {
                        println!("  [{i}] ID: {:?}", id.value);
                    }
                }
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}
