use std::env;

const ACCESS_TOKEN: &'static str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var(ACCESS_TOKEN)
        .expect(format!("{} not specified in environment", ACCESS_TOKEN).as_str());

    let fio = fiocz_rs::Fio::new(token.as_str());

    // match fio.import_transactions(transactions).await {
    //     Ok(v) => {
    //         println!("Imported transactions: {}", v)
    //     }
    //     Err(e) => {
    //         println!("Failed to import transactions {:?}", e)
    //     }
    // };

    // return;
    // match fio.movements_in_period("2022-08-01", "2022-09-01").await {
    //     Ok(statement) => {
    //         println!("Got movements containing {} transactions", statement.account_statement.transaction_list.transaction.len())
    //     }
    //     Err(e) => {
    //         println!("Failed to get account movements: {:?}", e)
    //     }
    // }
    // return;

    // match fio.statements("2022", "01").await {
    //     Ok(statement) => {
    //         println!("Got statement containing {} transactions", statement.account_statement.transaction_list.transaction.len())
    //     }
    //     Err(e) => {
    //         println!("Failed to get account statement: {:?}", e)
    //     }
    // }
    // return;

    // match fio.set_last_id("24230217199").await {
    //     Ok(_) => {
    //         println!("Set new stop")
    //     }
    //     Err(e) => {
    //         println!("Failed to get newest account movements: {:?}", e)
    //     }
    // }

    match fio.movements_since_last().await {
        Ok(statement) => {
            println!(
                "Got newest movements containing {} transactions",
                statement
                    .account_statement
                    .transaction_list
                    .transaction
                    .len()
            )
        }
        Err(e) => {
            println!("Failed to get newest account movements: {:?}", e)
        }
    }

    match fio.last_statement_id().await {
        Ok(id) => {
            println!("Got last statement id {:?} ", id)
        }
        Err(e) => {
            println!("Failed to get last statement id: {:?}", e)
        }
    }
}
