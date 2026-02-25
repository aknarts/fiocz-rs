//! Types for transaction import
mod xml;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Details of charges for foreign transactions
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum DetailsOfCharges {
    /// All charges paid by sender (OUR) - code 470501
    #[serde(rename = "470501")]
    Sender,
    /// All charges paid by receiver (BEN) - code 470502
    #[serde(rename = "470502")]
    Receiver,
    /// Each party pays own charges (SHA) - code 470503
    #[serde(rename = "470503")]
    Shared,
}

impl fmt::Display for DetailsOfCharges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sender => write!(f, "470501"),
            Self::Receiver => write!(f, "470502"),
            Self::Shared => write!(f, "470503"),
        }
    }
}

/// Payment type for domestic transactions
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum DomesticPaymentType {
    /// Standard payment - code 431001
    #[serde(rename = "431001")]
    Standard,
    /// Priority payment - code 431005
    #[serde(rename = "431005")]
    Priority,
    /// Direct debit order - code 431022
    #[serde(rename = "431022")]
    DirectDebit,
}

impl fmt::Display for DomesticPaymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "431001"),
            Self::Priority => write!(f, "431005"),
            Self::DirectDebit => write!(f, "431022"),
        }
    }
}

/// Payment type for euro (T2) transactions
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum EuroPaymentType {
    /// Standard payment - code 431008
    #[serde(rename = "431008")]
    Standard,
    /// Priority payment - code 431009
    #[serde(rename = "431009")]
    Priority,
    /// Instant payment - code 431018
    #[serde(rename = "431018")]
    Instant,
}

impl fmt::Display for EuroPaymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "431008"),
            Self::Priority => write!(f, "431009"),
            Self::Instant => write!(f, "431018"),
        }
    }
}

/// Domestic transaction data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DomesticTransaction {
    /// Account from
    pub account_from: String,
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: Decimal,
    /// Account to
    pub account_to: String,
    /// Bank code
    pub bank_code: String,
    /// Constant symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<String>,
    /// Variable symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs: Option<String>,
    /// Specific symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<String>,
    /// Date of transaction
    pub date: String,
    /// Message for recipient
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_for_recipient: Option<String>,
    /// Comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Payment reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reason: Option<String>,
    /// Payment type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<DomesticPaymentType>,
}

/// Euro (T2) transaction data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct T2Transaction {
    /// Account from
    pub account_from: String,
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: Decimal,
    /// Account to
    pub account_to: String,
    /// BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// Constant symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks: Option<String>,
    /// Variable symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs: Option<String>,
    /// Specific symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss: Option<String>,
    /// Date of transaction
    pub date: String,
    /// Beneficiary name
    pub benef_name: String,
    /// Beneficiary street
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benef_street: Option<String>,
    /// Beneficiary city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benef_city: Option<String>,
    /// Beneficiary country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benef_country: Option<String>,
    /// Remittance info 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info1: Option<String>,
    /// Remittance info 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info2: Option<String>,
    /// Remittance info 3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info3: Option<String>,
    /// Comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Payment reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_reason: Option<String>,
    /// Payment type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<EuroPaymentType>,
}

/// Foreign transaction data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForeignTransaction {
    /// Account from
    pub account_from: String,
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: Decimal,
    /// Account to
    pub account_to: String,
    /// BIC (mandatory for foreign transactions)
    pub bic: String,
    /// Date of transaction
    pub date: String,
    /// Beneficiary name
    pub benef_name: String,
    /// Beneficiary street (mandatory for foreign transactions)
    pub benef_street: String,
    /// Beneficiary city (mandatory for foreign transactions)
    pub benef_city: String,
    /// Beneficiary country (mandatory for foreign transactions)
    pub benef_country: String,
    /// Remittance info 1 (mandatory for foreign transactions)
    pub remittance_info1: String,
    /// Remittance info 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info2: Option<String>,
    /// Remittance info 3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info3: Option<String>,
    /// Remittance info 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info4: Option<String>,
    /// Comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Payment reason
    pub payment_reason: String,
    /// Details of charges
    pub details_of_charges: DetailsOfCharges,
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Type {
    /// Domestic transaction
    Domestic(DomesticTransaction),
    /// Euro transaction
    Euro(T2Transaction),
    /// Foreign transaction
    Foreign(ForeignTransaction),
}

/// Import transactions
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Import {
    /// List of transaction orders
    pub orders: Vec<Type>,
}

impl Import {
    /// Create new import
    #[must_use]
    pub const fn new() -> Self {
        Self { orders: vec![] }
    }
    /// Create new import builder
    #[must_use]
    pub fn builder() -> ImportBuilder {
        ImportBuilder::new()
    }
}

impl Default for Import {
    fn default() -> Self {
        Self::new()
    }
}

/// Import builder
pub struct ImportBuilder {
    domestic: Vec<DomesticTransaction>,
    euro: Vec<T2Transaction>,
    foreign: Vec<ForeignTransaction>,
}

impl ImportBuilder {
    /// Create new import builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            domestic: vec![],
            euro: vec![],
            foreign: vec![],
        }
    }

    /// Add domestic transaction
    pub fn domestic(&mut self, transaction: DomesticTransaction) -> &mut Self {
        self.domestic.push(transaction);
        self
    }

    /// Add Euro transaction
    pub fn euro(&mut self, transaction: T2Transaction) -> &mut Self {
        self.euro.push(transaction);
        self
    }

    /// Add foreign transaction
    pub fn foreign(&mut self, transaction: ForeignTransaction) -> &mut Self {
        self.foreign.push(transaction);
        self
    }

    /// Build import
    pub fn build(&mut self) -> Import {
        let mut import = Import::new();
        for t in self.domestic.drain(..) {
            import.orders.push(Type::Domestic(t));
        }
        for t in self.euro.drain(..) {
            import.orders.push(Type::Euro(t));
        }
        for t in self.foreign.drain(..) {
            import.orders.push(Type::Foreign(t));
        }
        import
    }
}

impl Default for ImportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
