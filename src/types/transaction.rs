//! Types for transaction
use serde::{Deserialize, Serialize};

/// Import transactions
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Import {
    /// List of transaction orders
    pub orders: Vec<TransactionType>,
}

impl Import {
    /// Create new import
    pub fn new() -> Self {
        Import { orders: vec![] }
    }
    /// Create new import builder
    pub fn builder() -> ImportBuilder {
        ImportBuilder::new()
    }
}

impl Import {
    /// Convert import to XML
    pub fn to_xml(&self) -> String {
        let mut result = Vec::new();
        result.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
        result.push("<Import xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://www.fio.cz/schema/importIB.xsd\">".to_string());
        result.push("<Orders>".to_string());
        self.orders.iter().for_each(|t| {
            if let TransactionType::DomesticTransaction { account_from, currency, amount, account_to, bank_code, ks, vs, ss, date, message_for_recipient, comment, payment_reason, payment_type } = t {
                result.push("<DomesticTransaction>".to_string());
                result.push(format!("<accountFrom>{}</accountFrom>", account_from));
                result.push(format!("<currency>{}</currency>", currency));
                result.push(format!("<amount>{}</amount>", amount));
                result.push(format!("<accountTo>{}</accountTo>", account_to));
                result.push(format!("<bankCode>{}</bankCode>", bank_code));
                if let Some(ks) = ks {
                    result.push(format!("<ks>{}</ks>", ks));
                }
                if let Some(vs) = vs {
                    result.push(format!("<vs>{}</vs>", vs));
                }
                if let Some(ss) = ss {
                    result.push(format!("<ss>{}</ss>", ss));
                }
                result.push(format!("<date>{}</date>", date));
                if let Some(message_for_recipient) = message_for_recipient {
                    result.push(format!("<messageForRecipient>{}</messageForRecipient>", message_for_recipient));
                }
                if let Some(comment) = comment {
                    result.push(format!("<comment>{}</comment>", comment));
                }
                if let Some(payment_reason) = payment_reason {
                    result.push(format!("<paymentReason>{}</paymentReason>", payment_reason));
                }
                if let Some(payment_type) = payment_type {
                    result.push(format!("<paymentType>{}</paymentType>", payment_type));
                }
                result.push("</DomesticTransaction>".to_string());
            }
        });
        self.orders.iter().for_each(|t| {
            if let TransactionType::T2Transaction { account_from, currency, amount, account_to, bic, ks, vs, ss, date, benef_name, benef_street, benef_city, benef_country, remittance_info1, remittance_info2, remittance_info3, comment, payment_reason, payment_type } = t {
                result.push("<T2Transaction>".to_string());
                result.push(format!("<accountFrom>{}</accountFrom>", account_from));
                result.push(format!("<currency>{}</currency>", currency));
                result.push(format!("<amount>{}</amount>", amount));
                result.push(format!("<accountTo>{}</accountTo>", account_to));
                if let Some(bic) = bic {
                    result.push(format!("<bic>{}</bic>", bic));
                }
                if let Some(ks) = ks {
                    result.push(format!("<ks>{}</ks>", ks));
                }
                if let Some(vs) = vs {
                    result.push(format!("<vs>{}</vs>", vs));
                }
                if let Some(ss) = ss {
                    result.push(format!("<ss>{}</ss>", ss));
                }
                result.push(format!("<date>{}</date>", date));
                result.push(format!("<benefName>{}</benefName>", benef_name));
                if let Some(benef_street) = benef_street {
                    result.push(format!("<benefStreet>{}</benefStreet>", benef_street));
                }
                if let Some(benef_city) = benef_city {
                    result.push(format!("<benefCity>{}</benefCity>", benef_city));
                }
                if let Some(benef_country) = benef_country {
                    result.push(format!("<benefCountry>{}</benefCountry>", benef_country));
                }
                if let Some(remittance_info1) = remittance_info1 {
                    result.push(format!("<remittanceInfo1>{}</remittanceInfo1>", remittance_info1));
                }
                if let Some(remittance_info2) = remittance_info2 {
                    result.push(format!("<remittanceInfo2>{}</remittanceInfo2>", remittance_info2));
                }
                if let Some(remittance_info3) = remittance_info3 {
                    result.push(format!("<remittanceInfo3>{}</remittanceInfo3>", remittance_info3));
                }
                if let Some(comment) = comment {
                    result.push(format!("<comment>{}</comment>", comment));
                }
                if let Some(payment_reason) = payment_reason {
                    result.push(format!("<paymentReason>{}</paymentReason>", payment_reason));
                }
                if let Some(payment_type) = payment_type {
                    result.push(format!("<paymentType>{}</paymentType>", payment_type));
                }
                result.push("</T2Transaction>".to_string());
            }
        });
        self.orders.iter().for_each(|t| {
            if let TransactionType::ForeignTransaction { account_from, currency, amount, account_to, bic, date, benef_name, benef_street, benef_city, benef_country, remittance_info1, remittance_info2, remittance_info3, remittance_info4, comment, payment_reason, details_of_charges } = t {
                result.push("<ForeignTransaction>".to_string());
                result.push(format!("<accountFrom>{}</accountFrom>", account_from));
                result.push(format!("<currency>{}</currency>", currency));
                result.push(format!("<amount>{}</amount>", amount));
                result.push(format!("<accountTo>{}</accountTo>", account_to));
                if let Some(bic) = bic {
                    result.push(format!("<bic>{}</bic>", bic));
                }
                result.push(format!("<date>{}</date>", date));
                result.push(format!("<benefName>{}</benefName>", benef_name));
                if let Some(benef_street) = benef_street {
                    result.push(format!("<benefStreet>{}</benefStreet>", benef_street));
                }
                if let Some(benef_city) = benef_city {
                    result.push(format!("<benefCity>{}</benefCity>", benef_city));
                }
                if let Some(benef_country) = benef_country {
                    result.push(format!("<benefCountry>{}</benefCountry>", benef_country));
                }
                if let Some(remittance_info1) = remittance_info1 {
                    result.push(format!("<remittanceInfo1>{}</remittanceInfo1>", remittance_info1));
                }
                if let Some(remittance_info2) = remittance_info2 {
                    result.push(format!("<remittanceInfo2>{}</remittanceInfo2>", remittance_info2));
                }
                if let Some(remittance_info3) = remittance_info3 {
                    result.push(format!("<remittanceInfo3>{}</remittanceInfo3>", remittance_info3));
                }
                if let Some(remittance_info4) = remittance_info4 {
                    result.push(format!("<remittanceInfo4>{}</remittanceInfo4>", remittance_info4));
                }
                if let Some(comment) = comment {
                    result.push(format!("<comment>{}</comment>", comment));
                }
                result.push(format!("<paymentReason>{}</paymentReason>", payment_reason));
                result.push(format!("<detailsOfCharges>{}</detailsOfCharges>", details_of_charges));
                result.push("</ForeignTransaction>".to_string());
            }
        });
        result.push("</Orders>".to_string());
        result.push("</Import>".to_string());
        result.join("")
    }
}

/// Import builder
pub struct ImportBuilder {
    domestic: Vec<TransactionType>,
    euro: Vec<TransactionType>,
    foreign: Vec<TransactionType>,
}

impl ImportBuilder {
    /// Create new import builder
    pub fn new() -> Self {
        return Self {
            domestic: vec![],
            euro: vec![],
            foreign: vec![],
        };
    }

    /// Add domestic transaction
    pub fn domestic(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::DomesticTransaction { .. } = transaction {
            self.domestic.push(transaction);
        }
        self
    }

    /// Add Euro transaction
    pub fn euro(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::T2Transaction { .. } = transaction {
            self.euro.push(transaction);
        }
        self
    }

    /// Add foreign transaction
    pub fn foreign(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::ForeignTransaction { .. } = transaction {
            self.foreign.push(transaction);
        }
        self
    }

    /// Build import
    pub fn build(&mut self) -> Import {
        let mut import = Import::new();
        for t in &self.domestic {
            import.orders.push(t.clone())
        }
        for t in &self.euro {
            import.orders.push(t.clone())
        }
        for t in &self.foreign {
            import.orders.push(t.clone())
        }
        import
    }
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TransactionType {
    /// Domestic transaction
    #[serde(rename_all = "camelCase")]
    DomesticTransaction {
        /// Account from
        account_from: String,
        /// Currency
        currency: String,
        /// Amount
        amount: f64,
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
        amount: f64,
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
        amount: f64,
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