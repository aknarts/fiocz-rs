//! Account statement types
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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

/// Last statement identifier
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LastStatementId {
    /// Year of the statement
    pub year: String,
    /// Statement ID
    pub id: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn transaction_data_enum_deserialize_integer() {
        let json = "42";
        let result: Result<TransactionDataEnum, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TransactionDataEnum::Integer(42));
    }

    #[test]
    fn transaction_data_enum_deserialize_string() {
        let json = "\"hello\"";
        let result: Result<TransactionDataEnum, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            TransactionDataEnum::String("hello".to_string())
        );
    }

    #[test]
    fn transaction_data_enum_deserialize_decimal() {
        let json = "123.45";
        let result: Result<TransactionDataEnum, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            TransactionDataEnum::Decimal(Decimal::new(12345, 2))
        );
    }

    #[test]
    fn last_statement_id_creation() {
        let id = LastStatementId {
            year: "2024".to_string(),
            id: "01".to_string(),
        };
        assert_eq!(id.year, "2024");
        assert_eq!(id.id, "01");
    }

    #[test]
    fn last_statement_id_clone_eq() {
        let id1 = LastStatementId {
            year: "2024".to_string(),
            id: "01".to_string(),
        };
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn transaction_data_enum_round_trip_integer() {
        let original = TransactionDataEnum::Integer(99);
        let json = serde_json::to_string(&original);
        assert!(json.is_ok());
        let deserialized: Result<TransactionDataEnum, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), original);
    }

    #[test]
    fn transaction_data_enum_round_trip_string() {
        let original = TransactionDataEnum::String("test value".to_string());
        let json = serde_json::to_string(&original);
        assert!(json.is_ok());
        let deserialized: Result<TransactionDataEnum, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap(), original);
    }
}
