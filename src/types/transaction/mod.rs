//! Types for transaction
mod xml;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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
    pub payment_type: Option<String>,
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
    pub payment_type: Option<String>,
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
    /// BIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
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
    /// Remittance info 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remittance_info4: Option<String>,
    /// Comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Payment reason
    pub payment_reason: String,
    /// Details of charges
    pub details_of_charges: String,
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
        for t in &self.domestic {
            import.orders.push(Type::Domestic(t.clone()));
        }
        for t in &self.euro {
            import.orders.push(Type::Euro(t.clone()));
        }
        for t in &self.foreign {
            import.orders.push(Type::Foreign(t.clone()));
        }
        import
    }
}

impl Default for ImportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn sample_domestic() -> DomesticTransaction {
        DomesticTransaction {
            account_from: "2345678901".to_string(),
            currency: "CZK".to_string(),
            amount: Decimal::new(10050, 2),
            account_to: "1234567890".to_string(),
            bank_code: "0800".to_string(),
            ks: None,
            vs: Some("1234567890".to_string()),
            ss: None,
            date: "2024-01-15".to_string(),
            message_for_recipient: Some("Test payment".to_string()),
            comment: None,
            payment_reason: None,
            payment_type: None,
        }
    }

    fn sample_euro() -> T2Transaction {
        T2Transaction {
            account_from: "CZ1234567890".to_string(),
            currency: "EUR".to_string(),
            amount: Decimal::new(5000, 2),
            account_to: "DE89370400440532013000".to_string(),
            bic: Some("COBADEFFXXX".to_string()),
            ks: None,
            vs: None,
            ss: None,
            date: "2024-02-01".to_string(),
            benef_name: "Test Recipient".to_string(),
            benef_street: None,
            benef_city: None,
            benef_country: None,
            remittance_info1: Some("Invoice 123".to_string()),
            remittance_info2: None,
            remittance_info3: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }
    }

    fn sample_foreign() -> ForeignTransaction {
        ForeignTransaction {
            account_from: "CZ1234567890".to_string(),
            currency: "USD".to_string(),
            amount: Decimal::new(25000, 2),
            account_to: "US12345678".to_string(),
            bic: None,
            date: "2024-03-15".to_string(),
            benef_name: "Foreign Corp".to_string(),
            benef_street: None,
            benef_city: None,
            benef_country: Some("US".to_string()),
            remittance_info1: None,
            remittance_info2: None,
            remittance_info3: None,
            remittance_info4: None,
            comment: None,
            payment_reason: "110".to_string(),
            details_of_charges: "SHA".to_string(),
        }
    }

    #[test]
    fn builder_empty_produces_no_orders() {
        let import = ImportBuilder::new().build();
        assert!(import.orders.is_empty());
    }

    #[test]
    fn builder_domestic_adds_transaction() {
        let import = ImportBuilder::new().domestic(sample_domestic()).build();
        assert_eq!(import.orders.len(), 1);
        assert!(matches!(import.orders[0], Type::Domestic(_)));
    }

    #[test]
    fn builder_euro_adds_transaction() {
        let import = ImportBuilder::new().euro(sample_euro()).build();
        assert_eq!(import.orders.len(), 1);
        assert!(matches!(import.orders[0], Type::Euro(_)));
    }

    #[test]
    fn builder_foreign_adds_transaction() {
        let import = ImportBuilder::new().foreign(sample_foreign()).build();
        assert_eq!(import.orders.len(), 1);
        assert!(matches!(import.orders[0], Type::Foreign(_)));
    }

    #[test]
    fn builder_chaining_all_types() {
        let import = ImportBuilder::new()
            .domestic(sample_domestic())
            .euro(sample_euro())
            .foreign(sample_foreign())
            .build();
        assert_eq!(import.orders.len(), 3);
        assert!(matches!(import.orders[0], Type::Domestic(_)));
        assert!(matches!(import.orders[1], Type::Euro(_)));
        assert!(matches!(import.orders[2], Type::Foreign(_)));
    }

    #[test]
    fn import_default_is_empty() {
        let import = Import::default();
        assert!(import.orders.is_empty());
    }
}
