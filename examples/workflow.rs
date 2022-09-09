use std::env;

const ACCESS_TOKEN: &'static str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() {
    let token = env::var(ACCESS_TOKEN).expect(format!("{} not specified in environment", ACCESS_TOKEN).as_str());

    let fio = fiocz_rs::Fio::new(token.as_str());
    match fio.get_movements_in_period("2022-08-01", "2022-09-01").await {
        Ok(statement) => {
            println!("Got statement containing {} transactions", statement.account_statement.transaction_list.transaction.len())
        }
        Err(e) => {
            println!("Failed to get account statement: {:?}", e)
        }
    }
}
