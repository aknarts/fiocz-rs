//! Fetch account movements for a date range.
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example movements_in_period -- 2024-01-01 2024-02-01 [format]
//!
//! The optional format argument can be: json, xml, csv, gpc, html, ofx
//! If omitted, fetches JSON and prints parsed transaction count.

use std::env;

use fiocz_rs::types::ExportFormat;
use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <start-date> <end-date> [format]", args[0]);
        eprintln!("  Dates in YYYY-MM-DD format");
        eprintln!("  Optional format: json, xml, csv, gpc, html, ofx");
        std::process::exit(1);
    }

    let start = &args[1];
    let end = &args[2];
    let fio = Fio::new(&token);

    if let Some(fmt) = args.get(3) {
        let format = match fmt.as_str() {
            "json" => ExportFormat::Json,
            "xml" => ExportFormat::Xml,
            "csv" => ExportFormat::Csv,
            "gpc" => ExportFormat::Gpc,
            "html" => ExportFormat::Html,
            "ofx" => ExportFormat::Ofx,
            other => {
                eprintln!("Unknown format: {other}");
                std::process::exit(1);
            }
        };
        match fio.movements_in_period_raw(start, end, format).await {
            Ok(body) => println!("{body}"),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    } else {
        match fio.movements_in_period(start, end).await {
            Ok(stmt) => {
                let info = &stmt.account_statement.info;
                let txns = &stmt.account_statement.transaction_list.transaction;
                println!("Account: {} ({})", info.account_id, info.currency);
                println!("Period: {} — {}", info.date_start, info.date_end);
                println!("Opening balance: {}", info.opening_balance);
                println!("Closing balance: {}", info.closing_balance);
                println!("Transactions: {}", txns.len());
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}
