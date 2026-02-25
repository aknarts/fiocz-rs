//! Set the download bookmark (cursor) by movement ID or date.
//!
//! Usage:
//!   ACCESS_TOKEN=xxx cargo run --example set_bookmark -- id 24230217199
//!   ACCESS_TOKEN=xxx cargo run --example set_bookmark -- date 2024-01-15

use std::env;

use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN env var required");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <id|date> <value>", args[0]);
        std::process::exit(1);
    }

    let fio = Fio::new(&token);

    match args[1].as_str() {
        "id" => match fio.set_last_id(&args[2]).await {
            Ok(()) => println!("Bookmark set to movement ID: {}", args[2]),
            Err(e) => eprintln!("Error: {e:?}"),
        },
        "date" => match fio.set_last_date(&args[2]).await {
            Ok(()) => println!("Bookmark set to date: {}", args[2]),
            Err(e) => eprintln!("Error: {e:?}"),
        },
        other => {
            eprintln!("Unknown bookmark type: {other} (use 'id' or 'date')");
            std::process::exit(1);
        }
    }
}
