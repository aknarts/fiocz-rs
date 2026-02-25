//! Merchant card transaction types
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Merchant statement response wrapper
///
/// Returned by the merchant card transactions endpoint for POS terminal
/// and payment gateway transactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantStatement {
    /// Account and period info
    pub info: MerchantInfo,
    /// Transaction list
    pub transaction_list: MerchantTransactionList,
}

/// Merchant account info
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantInfo {
    /// Account ID
    pub account_id: String,
    /// Bank ID (4 digit BBAN code)
    pub bank_id: String,
    /// Currency (ISO 4217)
    pub currency: String,
    /// IBAN
    pub iban: String,
    /// BIC (ISO 9362)
    pub bic: String,
    /// Start of selected period
    pub date_start: String,
    /// End of selected period
    pub date_end: String,
}

/// Merchant transaction list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantTransactionList {
    /// Card transactions
    pub transaction: Vec<MerchantTransaction>,
}

/// Single merchant card transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantTransaction {
    /// Unique operation ID
    pub operation_id: i64,
    /// Unique order ID
    pub order_id: i64,
    /// Date of settlement
    pub date: String,
    /// Batch settlement amount
    pub amount: Decimal,
    /// Note / description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Branch name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Transaction identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Device ID (terminal/gateway identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Transaction date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_date_time: Option<String>,
    /// Authorization number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autorization_number: Option<String>,
    /// Card number (masked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// Individual transaction amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<Decimal>,
    /// Transaction currency (ISO 4217)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currency: Option<String>,
    /// Transaction type (ON_US = Fio card, DOMESTIC = Czech card, FOREIGN = foreign card)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    /// Card issuer (e.g. MASTERCARD, VISA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuer: Option<String>,
    /// Total fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fees: Option<Decimal>,
    /// Fio bank fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fio_fee: Option<Decimal>,
    /// Interchange fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interchange_fee: Option<Decimal>,
    /// Card association fee (note: API uses "asosiation" spelling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_asosiation_fee: Option<Decimal>,
    /// Flexible commission / card issuer fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_commission: Option<Decimal>,
    /// Settlement status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<bool>,
    /// Settlement date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<String>,
    /// Variable symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs: Option<String>,
}
