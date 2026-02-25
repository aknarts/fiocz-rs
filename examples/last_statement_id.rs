//! Get the last created official statement number.
//!
//! Usage: ACCESS_TOKEN=xxx cargo run --example last_statement_id

use std::env;

use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let fio = Fio::new(&token);

    match fio.last_statement_id().await {
        Ok(id) => println!("Last statement: year={}, id={}", id.year, id.id),
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
