//! Fetch merchant card transactions (POS terminal / payment gateway).
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example merchant -- 2024-01-01 2024-02-01

use std::env;

use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <start-date> <end-date>", args[0]);
        std::process::exit(1);
    }

    let fio = Fio::new(&token);

    match fio.merchant_transactions_raw(&args[1], &args[2]).await {
        Ok(xml) => println!("{xml}"),
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
