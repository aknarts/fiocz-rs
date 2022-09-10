use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    pub account_statement: AccountStatement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatement {
    pub info: Info,
    pub transaction_list: TransactionList,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub account_id: String,
    pub bank_id: String,
    pub currency: String,
    pub iban: String,
    pub bic: String,
    pub opening_balance: f64,
    pub closing_balance: f64,
    pub date_start: String,
    pub date_end: String,
    pub year_list: Value,
    pub id_list: Option<i64>,
    pub id_from: Option<i64>,
    pub id_to: Option<i64>,
    pub id_last_download: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionList {
    pub transaction: Vec<HashMap<String, Option<TransactionData>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub value: TransactionDataEnum,
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionDataEnum {
    Integer(i64),
    String(String),
    Float(f64),
}