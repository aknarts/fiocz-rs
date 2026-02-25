//! XML generation for transaction import

use super::{DomesticTransaction, ForeignTransaction, Import, T2Transaction, Type};

impl Import {
    /// Convert import to XML
    #[must_use]
    pub fn to_xml(&self) -> String {
        let mut result = Vec::new();
        result.push("<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string());
        result.push("<Import xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://www.fio.cz/schema/importIB.xsd\">".to_string());
        result.push("<Orders>".to_string());
        self.orders.iter().for_each(|order| {
            if let Type::Domestic(t) = order {
                convert_domestic(&mut result, t);
            }
        });
        self.orders.iter().for_each(|order| {
            if let Type::Euro(t) = order {
                convert_euro(&mut result, t);
            }
        });
        self.orders.iter().for_each(|order| {
            if let Type::Foreign(t) = order {
                convert_foreign(&mut result, t);
            }
        });
        result.push("</Orders>".to_string());
        result.push("</Import>".to_string());
        result.join("")
    }
}

fn convert_foreign(result: &mut Vec<String>, t: &ForeignTransaction) {
    result.push("<ForeignTransaction>".to_string());
    result.push(format!("<accountFrom>{}</accountFrom>", t.account_from));
    result.push(format!("<currency>{}</currency>", t.currency));
    result.push(format!("<amount>{}</amount>", t.amount));
    result.push(format!("<accountTo>{}</accountTo>", t.account_to));
    result.push(format!("<bic>{}</bic>", t.bic));
    result.push(format!("<date>{}</date>", t.date));
    result.push(format!("<benefName>{}</benefName>", t.benef_name));
    result.push(format!("<benefStreet>{}</benefStreet>", t.benef_street));
    result.push(format!("<benefCity>{}</benefCity>", t.benef_city));
    result.push(format!("<benefCountry>{}</benefCountry>", t.benef_country));
    result.push(format!(
        "<remittanceInfo1>{}</remittanceInfo1>",
        t.remittance_info1
    ));
    if let Some(v) = &t.remittance_info2 {
        result.push(format!("<remittanceInfo2>{v}</remittanceInfo2>"));
    }
    if let Some(v) = &t.remittance_info3 {
        result.push(format!("<remittanceInfo3>{v}</remittanceInfo3>"));
    }
    if let Some(v) = &t.remittance_info4 {
        result.push(format!("<remittanceInfo4>{v}</remittanceInfo4>"));
    }
    if let Some(v) = &t.comment {
        result.push(format!("<comment>{v}</comment>"));
    }
    result.push(format!(
        "<paymentReason>{}</paymentReason>",
        t.payment_reason
    ));
    result.push(format!(
        "<detailsOfCharges>{}</detailsOfCharges>",
        t.details_of_charges
    ));
    result.push("</ForeignTransaction>".to_string());
}

fn convert_euro(result: &mut Vec<String>, t: &T2Transaction) {
    result.push("<T2Transaction>".to_string());
    result.push(format!("<accountFrom>{}</accountFrom>", t.account_from));
    result.push(format!("<currency>{}</currency>", t.currency));
    result.push(format!("<amount>{}</amount>", t.amount));
    result.push(format!("<accountTo>{}</accountTo>", t.account_to));
    if let Some(v) = &t.bic {
        result.push(format!("<bic>{v}</bic>"));
    }
    if let Some(v) = &t.ks {
        result.push(format!("<ks>{v}</ks>"));
    }
    if let Some(v) = &t.vs {
        result.push(format!("<vs>{v}</vs>"));
    }
    if let Some(v) = &t.ss {
        result.push(format!("<ss>{v}</ss>"));
    }
    result.push(format!("<date>{}</date>", t.date));
    result.push(format!("<benefName>{}</benefName>", t.benef_name));
    if let Some(v) = &t.benef_street {
        result.push(format!("<benefStreet>{v}</benefStreet>"));
    }
    if let Some(v) = &t.benef_city {
        result.push(format!("<benefCity>{v}</benefCity>"));
    }
    if let Some(v) = &t.benef_country {
        result.push(format!("<benefCountry>{v}</benefCountry>"));
    }
    if let Some(v) = &t.remittance_info1 {
        result.push(format!("<remittanceInfo1>{v}</remittanceInfo1>"));
    }
    if let Some(v) = &t.remittance_info2 {
        result.push(format!("<remittanceInfo2>{v}</remittanceInfo2>"));
    }
    if let Some(v) = &t.remittance_info3 {
        result.push(format!("<remittanceInfo3>{v}</remittanceInfo3>"));
    }
    if let Some(v) = &t.comment {
        result.push(format!("<comment>{v}</comment>"));
    }
    if let Some(v) = &t.payment_reason {
        result.push(format!("<paymentReason>{v}</paymentReason>"));
    }
    if let Some(v) = &t.payment_type {
        result.push(format!("<paymentType>{v}</paymentType>"));
    }
    result.push("</T2Transaction>".to_string());
}

fn convert_domestic(result: &mut Vec<String>, t: &DomesticTransaction) {
    result.push("<DomesticTransaction>".to_string());
    result.push(format!("<accountFrom>{}</accountFrom>", t.account_from));
    result.push(format!("<currency>{}</currency>", t.currency));
    result.push(format!("<amount>{}</amount>", t.amount));
    result.push(format!("<accountTo>{}</accountTo>", t.account_to));
    result.push(format!("<bankCode>{}</bankCode>", t.bank_code));
    if let Some(v) = &t.ks {
        result.push(format!("<ks>{v}</ks>"));
    }
    if let Some(v) = &t.vs {
        result.push(format!("<vs>{v}</vs>"));
    }
    if let Some(v) = &t.ss {
        result.push(format!("<ss>{v}</ss>"));
    }
    result.push(format!("<date>{}</date>", t.date));
    if let Some(v) = &t.message_for_recipient {
        result.push(format!("<messageForRecipient>{v}</messageForRecipient>"));
    }
    if let Some(v) = &t.comment {
        result.push(format!("<comment>{v}</comment>"));
    }
    if let Some(v) = &t.payment_reason {
        result.push(format!("<paymentReason>{v}</paymentReason>"));
    }
    if let Some(v) = &t.payment_type {
        result.push(format!("<paymentType>{v}</paymentType>"));
    }
    result.push("</DomesticTransaction>".to_string());
}

#[cfg(test)]
mod tests {
    use crate::types::transaction::*;
    use rust_decimal::Decimal;

    #[test]
    fn xml_empty_import() {
        let xml = Import::new().to_xml();
        assert!(xml.contains("<Orders></Orders>"));
    }

    #[test]
    fn xml_domestic() {
        let mut i = Import::new();
        i.orders.push(Type::Domestic(DomesticTransaction {
            account_from: "1".into(),
            currency: "CZK".into(),
            amount: Decimal::new(500, 0),
            account_to: "2".into(),
            bank_code: "0800".into(),
            ks: None,
            vs: Some("123".into()),
            ss: None,
            date: "2024-01-01".into(),
            message_for_recipient: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }));
        let xml = i.to_xml();
        assert!(xml.contains("<DomesticTransaction>"));
        assert!(xml.contains("<vs>123</vs>"));
        assert!(!xml.contains("<ks>"));
    }

    #[test]
    fn xml_euro() {
        let mut i = Import::new();
        i.orders.push(Type::Euro(T2Transaction {
            account_from: "1".into(),
            currency: "EUR".into(),
            amount: Decimal::new(100, 0),
            account_to: "2".into(),
            bic: Some("BIC".into()),
            ks: None,
            vs: None,
            ss: None,
            date: "2024-01-01".into(),
            benef_name: "N".into(),
            benef_street: None,
            benef_city: None,
            benef_country: None,
            remittance_info1: None,
            remittance_info2: None,
            remittance_info3: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }));
        let xml = i.to_xml();
        assert!(xml.contains("<T2Transaction>"));
        assert!(xml.contains("<bic>BIC</bic>"));
    }

    #[test]
    fn xml_foreign() {
        let mut i = Import::new();
        i.orders.push(Type::Foreign(ForeignTransaction {
            account_from: "1".into(),
            currency: "USD".into(),
            amount: Decimal::new(200, 0),
            account_to: "2".into(),
            bic: "BIC".into(),
            date: "2024-01-01".into(),
            benef_name: "N".into(),
            benef_street: "S".into(),
            benef_city: "C".into(),
            benef_country: "US".into(),
            remittance_info1: "R".into(),
            remittance_info2: None,
            remittance_info3: None,
            remittance_info4: None,
            comment: None,
            payment_reason: "110".into(),
            details_of_charges: DetailsOfCharges::Shared,
        }));
        let xml = i.to_xml();
        assert!(xml.contains("<ForeignTransaction>"));
        assert!(xml.contains("<detailsOfCharges>470503</detailsOfCharges>"));
        assert!(xml.contains("<bic>BIC</bic>"));
    }

    #[test]
    fn xml_ordering() {
        let mut i = Import::new();
        i.orders.push(Type::Domestic(DomesticTransaction {
            account_from: "1".into(),
            currency: "CZK".into(),
            amount: Decimal::new(1, 0),
            account_to: "2".into(),
            bank_code: "0800".into(),
            ks: None,
            vs: None,
            ss: None,
            date: "2024-01-01".into(),
            message_for_recipient: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }));
        i.orders.push(Type::Euro(T2Transaction {
            account_from: "1".into(),
            currency: "EUR".into(),
            amount: Decimal::new(1, 0),
            account_to: "2".into(),
            bic: None,
            ks: None,
            vs: None,
            ss: None,
            date: "2024-01-01".into(),
            benef_name: "N".into(),
            benef_street: None,
            benef_city: None,
            benef_country: None,
            remittance_info1: None,
            remittance_info2: None,
            remittance_info3: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }));
        i.orders.push(Type::Foreign(ForeignTransaction {
            account_from: "1".into(),
            currency: "USD".into(),
            amount: Decimal::new(1, 0),
            account_to: "2".into(),
            bic: "B".into(),
            date: "2024-01-01".into(),
            benef_name: "N".into(),
            benef_street: "S".into(),
            benef_city: "C".into(),
            benef_country: "US".into(),
            remittance_info1: "R".into(),
            remittance_info2: None,
            remittance_info3: None,
            remittance_info4: None,
            comment: None,
            payment_reason: "110".into(),
            details_of_charges: DetailsOfCharges::Sender,
        }));
        let xml = i.to_xml();
        let d = xml.find("<DomesticTransaction>").unwrap();
        let e = xml.find("<T2Transaction>").unwrap();
        let f = xml.find("<ForeignTransaction>").unwrap();
        assert!(d < e);
        assert!(e < f);
    }
}
