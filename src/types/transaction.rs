//! Types for transaction
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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

impl Import {
    /// Convert import to XML
    #[must_use]
    pub fn to_xml(&self) -> String {
        let mut result = Vec::new();
        result.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
        result.push("<Import xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://www.fio.cz/schema/importIB.xsd\">".to_string());
        result.push("<Orders>".to_string());
        self.orders.iter().for_each(|t| {
            Self::convert_domestic(&mut result, t);
        });
        self.orders.iter().for_each(|t| {
            Self::convert_euro(&mut result, t);
        });
        self.orders.iter().for_each(|t| {
            Self::convert_foreign(&mut result, t);
        });
        result.push("</Orders>".to_string());
        result.push("</Import>".to_string());
        result.join("")
    }

    fn convert_foreign(result: &mut Vec<String>, t: &Type) {
        if let Type::ForeignTransaction { account_from, currency, amount, account_to, bic, date, benef_name, benef_street, benef_city, benef_country, remittance_info1, remittance_info2, remittance_info3, remittance_info4, comment, payment_reason, details_of_charges } = t {
            result.push("<ForeignTransaction>".to_string());
            result.push(format!("<accountFrom>{account_from}</accountFrom>"));
            result.push(format!("<currency>{currency}</currency>"));
            result.push(format!("<amount>{amount}</amount>"));
            result.push(format!("<accountTo>{account_to}</accountTo>"));
            if let Some(bic) = bic {
                result.push(format!("<bic>{bic}</bic>"));
            }
            result.push(format!("<date>{date}</date>"));
            result.push(format!("<benefName>{benef_name}</benefName>"));
            if let Some(benef_street) = benef_street {
                result.push(format!("<benefStreet>{benef_street}</benefStreet>"));
            }
            if let Some(benef_city) = benef_city {
                result.push(format!("<benefCity>{benef_city}</benefCity>"));
            }
            if let Some(benef_country) = benef_country {
                result.push(format!("<benefCountry>{benef_country}</benefCountry>"));
            }
            if let Some(remittance_info1) = remittance_info1 {
                result.push(format!("<remittanceInfo1>{remittance_info1}</remittanceInfo1>"));
            }
            if let Some(remittance_info2) = remittance_info2 {
                result.push(format!("<remittanceInfo2>{remittance_info2}</remittanceInfo2>"));
            }
            if let Some(remittance_info3) = remittance_info3 {
                result.push(format!("<remittanceInfo3>{remittance_info3}</remittanceInfo3>"));
            }
            if let Some(remittance_info4) = remittance_info4 {
                result.push(format!("<remittanceInfo4>{remittance_info4}</remittanceInfo4>"));
            }
            if let Some(comment) = comment {
                result.push(format!("<comment>{comment}</comment>"));
            }
            result.push(format!("<paymentReason>{payment_reason}</paymentReason>"));
            result.push(format!("<detailsOfCharges>{details_of_charges}</detailsOfCharges>"));
            result.push("</ForeignTransaction>".to_string());
        }
    }

    fn convert_euro(result: &mut Vec<String>, t: &Type) {
        if let Type::T2Transaction { account_from, currency, amount, account_to, bic, ks, vs, ss, date, benef_name, benef_street, benef_city, benef_country, remittance_info1, remittance_info2, remittance_info3, comment, payment_reason, payment_type } = t {
            result.push("<T2Transaction>".to_string());
            result.push(format!("<accountFrom>{account_from}</accountFrom>"));
            result.push(format!("<currency>{currency}</currency>"));
            result.push(format!("<amount>{amount}</amount>"));
            result.push(format!("<accountTo>{account_to}</accountTo>"));
            if let Some(bic) = bic {
                result.push(format!("<bic>{bic}</bic>"));
            }
            if let Some(ks) = ks {
                result.push(format!("<ks>{ks}</ks>"));
            }
            if let Some(vs) = vs {
                result.push(format!("<vs>{vs}</vs>"));
            }
            if let Some(ss) = ss {
                result.push(format!("<ss>{ss}</ss>"));
            }
            result.push(format!("<date>{date}</date>"));
            result.push(format!("<benefName>{benef_name}</benefName>"));
            if let Some(benef_street) = benef_street {
                result.push(format!("<benefStreet>{benef_street}</benefStreet>"));
            }
            if let Some(benef_city) = benef_city {
                result.push(format!("<benefCity>{benef_city}</benefCity>"));
            }
            if let Some(benef_country) = benef_country {
                result.push(format!("<benefCountry>{benef_country}</benefCountry>"));
            }
            if let Some(remittance_info1) = remittance_info1 {
                result.push(format!("<remittanceInfo1>{remittance_info1}</remittanceInfo1>"));
            }
            if let Some(remittance_info2) = remittance_info2 {
                result.push(format!("<remittanceInfo2>{remittance_info2}</remittanceInfo2>"));
            }
            if let Some(remittance_info3) = remittance_info3 {
                result.push(format!("<remittanceInfo3>{remittance_info3}</remittanceInfo3>"));
            }
            if let Some(comment) = comment {
                result.push(format!("<comment>{comment}</comment>"));
            }
            if let Some(payment_reason) = payment_reason {
                result.push(format!("<paymentReason>{payment_reason}</paymentReason>"));
            }
            if let Some(payment_type) = payment_type {
                result.push(format!("<paymentType>{payment_type}</paymentType>"));
            }
            result.push("</T2Transaction>".to_string());
        }
    }

    fn convert_domestic(result: &mut Vec<String>, t: &Type) {
        if let Type::DomesticTransaction { account_from, currency, amount, account_to, bank_code, ks, vs, ss, date, message_for_recipient, comment, payment_reason, payment_type } = t {
            result.push("<DomesticTransaction>".to_string());
            result.push(format!("<accountFrom>{account_from}</accountFrom>"));
            result.push(format!("<currency>{currency}</currency>"));
            result.push(format!("<amount>{amount}</amount>"));
            result.push(format!("<accountTo>{account_to}</accountTo>"));
            result.push(format!("<bankCode>{bank_code}</bankCode>"));
            if let Some(ks) = ks {
                result.push(format!("<ks>{ks}</ks>"));
            }
            if let Some(vs) = vs {
                result.push(format!("<vs>{vs}</vs>"));
            }
            if let Some(ss) = ss {
                result.push(format!("<ss>{ss}</ss>"));
            }
            result.push(format!("<date>{date}</date>"));
            if let Some(message_for_recipient) = message_for_recipient {
                result.push(format!("<messageForRecipient>{message_for_recipient}</messageForRecipient>"));
            }
            if let Some(comment) = comment {
                result.push(format!("<comment>{comment}</comment>"));
            }
            if let Some(payment_reason) = payment_reason {
                result.push(format!("<paymentReason>{payment_reason}</paymentReason>"));
            }
            if let Some(payment_type) = payment_type {
                result.push(format!("<paymentType>{payment_type}</paymentType>"));
            }
            result.push("</DomesticTransaction>".to_string());
        }
    }
}

/// Import builder
pub struct ImportBuilder {
    domestic: Vec<Type>,
    euro: Vec<Type>,
    foreign: Vec<Type>,
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
    pub fn domestic(&mut self, transaction: Type) -> &mut Self {
        if let Type::DomesticTransaction { .. } = transaction {
            self.domestic.push(transaction);
        }
        self
    }

    /// Add Euro transaction
    pub fn euro(&mut self, transaction: Type) -> &mut Self {
        if let Type::T2Transaction { .. } = transaction {
            self.euro.push(transaction);
        }
        self
    }

    /// Add foreign transaction
    pub fn foreign(&mut self, transaction: Type) -> &mut Self {
        if let Type::ForeignTransaction { .. } = transaction {
            self.foreign.push(transaction);
        }
        self
    }

    /// Build import
    pub fn build(&mut self) -> Import {
        let mut import = Import::new();
        for t in &self.domestic {
            import.orders.push(t.clone());
        }
        for t in &self.euro {
            import.orders.push(t.clone());
        }
        for t in &self.foreign {
            import.orders.push(t.clone());
        }
        import
    }
}

impl Default for ImportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Type {
    /// Domestic transaction
    #[serde(rename_all = "camelCase")]
    DomesticTransaction {
        /// Account from
        account_from: String,
        /// Currency
        currency: String,
        /// Amount
        amount: Decimal,
        /// Account to
        account_to: String,
        /// Bank code
        bank_code: String,
        /// Constant symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        ks: Option<String>,
        /// Variable symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        vs: Option<String>,
        /// Specific symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        ss: Option<String>,
        /// Date of transaction
        date: String,
        /// Message for recipient
        #[serde(skip_serializing_if = "Option::is_none")]
        message_for_recipient: Option<String>,
        /// Comment
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        /// Payment reason
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_reason: Option<String>,
        /// Payment type
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_type: Option<String>,
    },
    /// Euro transaction
    #[serde(rename_all = "camelCase")]
    T2Transaction {
        /// Account from
        account_from: String,
        /// Currency
        currency: String,
        /// Amount
        amount: Decimal,
        /// Account to
        account_to: String,
        /// BIC
        #[serde(skip_serializing_if = "Option::is_none")]
        bic: Option<String>,
        /// Constant symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        ks: Option<String>,
        /// Variable symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        vs: Option<String>,
        /// Specific symbol
        #[serde(skip_serializing_if = "Option::is_none")]
        ss: Option<String>,
        /// Date of transaction
        date: String,
        /// Beneficiary name
        benef_name: String,
        /// Beneficiary street
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_street: Option<String>,
        /// Beneficiary city
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_city: Option<String>,
        /// Beneficiary country
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_country: Option<String>,
        /// Remittance info 1
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info1: Option<String>,
        /// Remittance info 2
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info2: Option<String>,
        /// Remittance info 3
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info3: Option<String>,
        /// Comment
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        /// Payment reason
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_reason: Option<String>,
        /// Payment type
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_type: Option<String>,
    },
    /// Foreign transaction
    #[serde(rename_all = "camelCase")]
    ForeignTransaction {
        /// Account from
        account_from: String,
        /// Currency
        currency: String,
        /// Amount
        amount: Decimal,
        /// Account to
        account_to: String,
        /// BIC
        #[serde(skip_serializing_if = "Option::is_none")]
        bic: Option<String>,
        /// Date of transaction
        date: String,
        /// Beneficiary name
        benef_name: String,
        /// Beneficiary street
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_street: Option<String>,
        /// Beneficiary city
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_city: Option<String>,
        /// Beneficiary country
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_country: Option<String>,
        /// Remittance info 1
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info1: Option<String>,
        /// Remittance info 2
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info2: Option<String>,
        /// Remittance info 3
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info3: Option<String>,
        /// Remittance info 4
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info4: Option<String>,
        /// Comment
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        /// Payment reason
        payment_reason: String,
        /// Details of charges
        details_of_charges: String,
    },
}
