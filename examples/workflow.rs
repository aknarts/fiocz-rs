use std::env;
use fiocz_rs::types::transaction::{Import, TestEnum, TransactionType};

const ACCESS_TOKEN: &'static str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var(ACCESS_TOKEN).expect(format!("{} not specified in environment", ACCESS_TOKEN).as_str());

    let fio = fiocz_rs::Fio::new(token.as_str());

    let mut builder = Import::builder();

    builder.domestic(TransactionType::DomesticTransaction {
        account_from: "2800478896".to_string(),
        currency: "CZK".to_string(),
        amount: 100.0,
        account_to: "2101179627".to_string(),
        bank_code: "2010".to_string(),
        ks: Some("0558".to_string()),
        vs: Some("1234567890".to_string()),
        ss: Some("1234567890".to_string()),
        date: "2022-09-25".to_string(),
        message_for_recipient: Some("Test".to_string()),
        comment: None,
        payment_reason: None,
        payment_type: Some("431001".to_string()),
    });
    builder.domestic(TransactionType::DomesticTransaction {
        account_from: "2800478896".to_string(),
        currency: "CZK".to_string(),
        amount: 100.0,
        account_to: "2101179627".to_string(),
        bank_code: "2010".to_string(),
        ks: Some("0558".to_string()),
        vs: Some("1234567890".to_string()),
        ss: Some("1234567890".to_string()),
        date: "2022-09-25".to_string(),
        message_for_recipient: Some("Test".to_string()),
        comment: None,
        payment_reason: None,
        payment_type: Some("431001".to_string()),
    });
    // builder.euro(TransactionType::T2Transaction {
    //     account_from: "2800478896".to_string(),
    //     currency: "EUR".to_string(),
    //     amount: 10.0,
    //     account_to: "AT611904300234573201".to_string(),
    //     bic: Some("ABAGATWWXXX".to_string()),
    //     ks: None,
    //     vs: None,
    //     ss: None,
    //     date: "2022-09-25".to_string(),
    //     benef_name: "Hans Gruber".to_string(),
    //     benef_street: None,
    //     benef_city: None,
    //     benef_country: Some("AT".to_string()),
    //     remittance_info1: None,
    //     remittance_info2: None,
    //     remittance_info3: None,
    //     comment: None,
    //     payment_reason: None,
    //     payment_type: Some("431008".to_string()),
    // });
    // builder.foreign(TransactionType::ForeignTransaction {
    //     account_from: "2800478896".to_string(),
    //     currency: "USD".to_string(),
    //     amount: 10.0,
    //     account_to: "PK36SCBL0000001123456702".to_string(),
    //     bic: Some("ALFHPKKAXXX".to_string()),
    //     date: "2022-09-25".to_string(),
    //     benef_name: "Amir Khan".to_string(),
    //     benef_street: None,
    //     benef_city: None,
    //     benef_country: Some("PK".to_string()),
    //     remittance_info1: None,
    //     remittance_info2: None,
    //     remittance_info3: None,
    //     remittance_info4: None,
    //     comment: None,
    //     payment_reason: "348".to_string(),
    //     details_of_charges: "470501".to_string(),
    // });

    let transactions = builder.build();

    // println!("{}", serde_xml_rs::to_string(&transactions).unwrap());

    let mut test = fiocz_rs::types::transaction::Test { val: vec![] };
    test.val.push(TestEnum::FirstEnum { content: "Test1".to_string() });
    test.val.push(TestEnum::SecondEnum { cont: "Test2".to_string() });
    println!("{}", serde_json::to_string(&test).unwrap());
    println!("{}", serde_xml_rs::to_string(&test).unwrap());
    // match fio.import_transactions(transactions).await {
    //     Ok(v) => {
    //         println!("Imported transactions: {}", v)
    //     }
    //     Err(e) => {
    //         println!("Failed to import transactions {:?}", e)
    //     }
    // };

    return;
    match fio.movements_in_period("2022-08-01", "2022-09-01").await {
        Ok(statement) => {
            println!("Got movements containing {} transactions", statement.account_statement.transaction_list.transaction.len())
        }
        Err(e) => {
            println!("Failed to get account movements: {:?}", e)
        }
    }

    match fio.statements("2022", "01").await {
        Ok(statement) => {
            println!("Got statement containing {} transactions", statement.account_statement.transaction_list.transaction.len())
        }
        Err(e) => {
            println!("Failed to get account statement: {:?}", e)
        }
    }

    match fio.set_last_id("24230217199").await {
        Ok(_) => {
            println!("Set new stop")
        }
        Err(e) => {
            println!("Failed to get newest account movements: {:?}", e)
        }
    }

    match fio.movements_since_last().await {
        Ok(statement) => {
            println!("Got newest movements containing {} transactions", statement.account_statement.transaction_list.transaction.len())
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
