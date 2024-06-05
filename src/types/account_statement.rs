//! Account statement types
use std::collections::HashMap;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Holder for account statement
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    /// Account statement
    pub account_statement: AccountStatement,
}

/// Account statement
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatement {
    /// Account info
    pub info: Info,
    /// Transaction list
    pub transaction_list: TransactionList,
}

/// Account info
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// Account ID
    pub account_id: String,
    /// Bank ID
    pub bank_id: String,
    /// Currency
    pub currency: String,
    /// IBAN
    pub iban: String,
    /// BIC
    pub bic: String,
    /// Opening balance
    pub opening_balance: Decimal,
    /// Closing balance
    pub closing_balance: Decimal,
    /// Date start
    pub date_start: String,
    /// Date end
    pub date_end: String,
    /// Year list
    pub year_list: Value,
    /// ID list
    pub id_list: Option<i64>,
    /// ID from
    pub id_from: Option<i64>,
    /// ID to
    pub id_to: Option<i64>,
    /// ID last download
    pub id_last_download: Option<i64>,
}

/// Transaction list
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionList {
    /// Transactions
    pub transaction: Vec<HashMap<String, Option<TransactionData>>>,
}

/// Transaction data
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    /// Transaction Data Value
    pub value: TransactionDataEnum,
    /// Transaction Data Name
    pub name: String,
    /// Transaction Data Id
    pub id: i64,
}

/// Transaction data enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionDataEnum {
    /// Transaction data int value
    Integer(i64),
    /// Transaction data string value
    String(String),
    /// Transaction data float value
    Decimal(Decimal),
}
