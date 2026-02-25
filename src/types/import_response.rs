//! Import response types
use rust_decimal::Decimal;

/// Response from the import endpoint
///
/// The FIO API returns an XML response after importing transactions.
/// This struct represents the parsed response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportResponse {
    /// Error code (0 = ok, 1 = validation error, 2 = warning, 11 = syntax error,
    /// 12 = empty import, 13 = file too large, 14 = empty file)
    pub error_code: i32,
    /// Unique batch/instruction ID assigned by the bank
    pub id_instruction: Option<String>,
    /// Status string (ok, error, warning, fatal)
    pub status: Option<String>,
    /// Sum of debit items in the batch
    pub sum_debet: Option<Decimal>,
    /// Sum of credit items in the batch
    pub sum_credit: Option<Decimal>,
}
