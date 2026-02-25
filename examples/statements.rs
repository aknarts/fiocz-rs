//! Fetch official account statement by year and ID.
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example statements -- 2024 1 [format]

use std::env;

use fiocz_rs::types::ExportFormat;
use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <year> <statement-id> [format]", args[0]);
        eprintln!("  format: json, xml, csv, pdf, sta, cba_xml, sba_xml");
        std::process::exit(1);
    }

    let year = &args[1];
    let id = &args[2];
    let fio = Fio::new(&token);

    if let Some(fmt) = args.get(3) {
        let format = match fmt.as_str() {
            "json" => ExportFormat::Json,
            "xml" => ExportFormat::Xml,
            "csv" => ExportFormat::Csv,
            "pdf" => ExportFormat::Pdf,
            "sta" => ExportFormat::Mt940,
            "cba_xml" => ExportFormat::CbaXml,
            "sba_xml" => ExportFormat::SbaXml,
            other => {
                eprintln!("Unknown format: {other}");
                std::process::exit(1);
            }
        };
        match fio.statements_raw(year, id, format).await {
            Ok(body) => println!("{body}"),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    } else {
        match fio.statements(year, id).await {
            Ok(stmt) => {
                let info = &stmt.account_statement.info;
                let txns = &stmt.account_statement.transaction_list.transaction;
                println!("Statement {}/{}", year, id);
                println!("Account: {} ({})", info.account_id, info.currency);
                println!("Transactions: {}", txns.len());
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}
