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
    if let Some(bic) = &t.bic {
        result.push(format!("<bic>{bic}</bic>"));
    }
    result.push(format!("<date>{}</date>", t.date));
    result.push(format!("<benefName>{}</benefName>", t.benef_name));
    if let Some(benef_street) = &t.benef_street {
        result.push(format!("<benefStreet>{benef_street}</benefStreet>"));
    }
    if let Some(benef_city) = &t.benef_city {
        result.push(format!("<benefCity>{benef_city}</benefCity>"));
    }
    if let Some(benef_country) = &t.benef_country {
        result.push(format!("<benefCountry>{benef_country}</benefCountry>"));
    }
    if let Some(remittance_info1) = &t.remittance_info1 {
        result.push(format!(
            "<remittanceInfo1>{remittance_info1}</remittanceInfo1>"
        ));
    }
    if let Some(remittance_info2) = &t.remittance_info2 {
        result.push(format!(
            "<remittanceInfo2>{remittance_info2}</remittanceInfo2>"
        ));
    }
    if let Some(remittance_info3) = &t.remittance_info3 {
        result.push(format!(
            "<remittanceInfo3>{remittance_info3}</remittanceInfo3>"
        ));
    }
    if let Some(remittance_info4) = &t.remittance_info4 {
        result.push(format!(
            "<remittanceInfo4>{remittance_info4}</remittanceInfo4>"
        ));
    }
    if let Some(comment) = &t.comment {
        result.push(format!("<comment>{comment}</comment>"));
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
    if let Some(bic) = &t.bic {
        result.push(format!("<bic>{bic}</bic>"));
    }
    if let Some(ks) = &t.ks {
        result.push(format!("<ks>{ks}</ks>"));
    }
    if let Some(vs) = &t.vs {
        result.push(format!("<vs>{vs}</vs>"));
    }
    if let Some(ss) = &t.ss {
        result.push(format!("<ss>{ss}</ss>"));
    }
    result.push(format!("<date>{}</date>", t.date));
    result.push(format!("<benefName>{}</benefName>", t.benef_name));
    if let Some(benef_street) = &t.benef_street {
        result.push(format!("<benefStreet>{benef_street}</benefStreet>"));
    }
    if let Some(benef_city) = &t.benef_city {
        result.push(format!("<benefCity>{benef_city}</benefCity>"));
    }
    if let Some(benef_country) = &t.benef_country {
        result.push(format!("<benefCountry>{benef_country}</benefCountry>"));
    }
    if let Some(remittance_info1) = &t.remittance_info1 {
        result.push(format!(
            "<remittanceInfo1>{remittance_info1}</remittanceInfo1>"
        ));
    }
    if let Some(remittance_info2) = &t.remittance_info2 {
        result.push(format!(
            "<remittanceInfo2>{remittance_info2}</remittanceInfo2>"
        ));
    }
    if let Some(remittance_info3) = &t.remittance_info3 {
        result.push(format!(
            "<remittanceInfo3>{remittance_info3}</remittanceInfo3>"
        ));
    }
    if let Some(comment) = &t.comment {
        result.push(format!("<comment>{comment}</comment>"));
    }
    if let Some(payment_reason) = &t.payment_reason {
        result.push(format!("<paymentReason>{payment_reason}</paymentReason>"));
    }
    if let Some(payment_type) = &t.payment_type {
        result.push(format!("<paymentType>{payment_type}</paymentType>"));
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
    if let Some(ks) = &t.ks {
        result.push(format!("<ks>{ks}</ks>"));
    }
    if let Some(vs) = &t.vs {
        result.push(format!("<vs>{vs}</vs>"));
    }
    if let Some(ss) = &t.ss {
        result.push(format!("<ss>{ss}</ss>"));
    }
    result.push(format!("<date>{}</date>", t.date));
    if let Some(message_for_recipient) = &t.message_for_recipient {
        result.push(format!(
            "<messageForRecipient>{message_for_recipient}</messageForRecipient>"
        ));
    }
    if let Some(comment) = &t.comment {
        result.push(format!("<comment>{comment}</comment>"));
    }
    if let Some(payment_reason) = &t.payment_reason {
        result.push(format!("<paymentReason>{payment_reason}</paymentReason>"));
    }
    if let Some(payment_type) = &t.payment_type {
        result.push(format!("<paymentType>{payment_type}</paymentType>"));
    }
    result.push("</DomesticTransaction>".to_string());
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

    #[test]
    fn xml_empty_import() {
        let import = Import::new();
        let xml = import.to_xml();
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<Orders></Orders>"));
        assert!(xml.ends_with("</Import>"));
    }

    #[test]
    fn xml_domestic_required_fields() {
        let mut import = Import::new();
        import.orders.push(Type::Domestic(DomesticTransaction {
            account_from: "111".to_string(),
            currency: "CZK".to_string(),
            amount: Decimal::new(500, 0),
            account_to: "222".to_string(),
            bank_code: "0800".to_string(),
            ks: None,
            vs: None,
            ss: None,
            date: "2024-01-01".to_string(),
            message_for_recipient: None,
            comment: None,
            payment_reason: None,
            payment_type: None,
        }));
        let xml = import.to_xml();
        assert!(xml.contains("<DomesticTransaction>"));
        assert!(xml.contains("<accountFrom>111</accountFrom>"));
        assert!(xml.contains("<bankCode>0800</bankCode>"));
        assert!(xml.contains("<amount>500</amount>"));
        assert!(xml.contains("</DomesticTransaction>"));
        // Optional fields should NOT appear
        assert!(!xml.contains("<ks>"));
        assert!(!xml.contains("<vs>"));
        assert!(!xml.contains("<comment>"));
    }

    #[test]
    fn xml_domestic_with_optional_fields() {
        let mut import = Import::new();
        import.orders.push(Type::Domestic(sample_domestic()));
        let xml = import.to_xml();
        assert!(xml.contains("<vs>1234567890</vs>"));
        assert!(xml.contains("<messageForRecipient>Test payment</messageForRecipient>"));
        assert!(!xml.contains("<ks>"));
    }

    #[test]
    fn xml_euro_transaction() {
        let mut import = Import::new();
        import.orders.push(Type::Euro(T2Transaction {
            account_from: "CZ111".to_string(),
            currency: "EUR".to_string(),
            amount: Decimal::new(5000, 2),
            account_to: "DE222".to_string(),
            bic: Some("COBADEFF".to_string()),
            ks: None,
            vs: None,
            ss: None,
            date: "2024-02-01".to_string(),
            benef_name: "Test".to_string(),
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
        let xml = import.to_xml();
        assert!(xml.contains("<T2Transaction>"));
        assert!(xml.contains("<bic>COBADEFF</bic>"));
        assert!(xml.contains("<benefName>Test</benefName>"));
        assert!(xml.contains("</T2Transaction>"));
    }

    #[test]
    fn xml_foreign_transaction() {
        let mut import = Import::new();
        import.orders.push(Type::Foreign(ForeignTransaction {
            account_from: "CZ111".to_string(),
            currency: "USD".to_string(),
            amount: Decimal::new(25000, 2),
            account_to: "US222".to_string(),
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
        }));
        let xml = import.to_xml();
        assert!(xml.contains("<ForeignTransaction>"));
        assert!(xml.contains("<benefCountry>US</benefCountry>"));
        assert!(xml.contains("<paymentReason>110</paymentReason>"));
        assert!(xml.contains("<detailsOfCharges>SHA</detailsOfCharges>"));
        assert!(xml.contains("</ForeignTransaction>"));
        assert!(!xml.contains("<bic>"));
    }

    #[test]
    fn xml_mixed_ordering() {
        let mut import = Import::new();
        import.orders.push(Type::Domestic(sample_domestic()));
        import.orders.push(Type::Euro(T2Transaction {
            account_from: "CZ111".to_string(),
            currency: "EUR".to_string(),
            amount: Decimal::new(100, 0),
            account_to: "DE222".to_string(),
            bic: None,
            ks: None,
            vs: None,
            ss: None,
            date: "2024-02-01".to_string(),
            benef_name: "Euro Recv".to_string(),
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
        import.orders.push(Type::Foreign(ForeignTransaction {
            account_from: "CZ111".to_string(),
            currency: "USD".to_string(),
            amount: Decimal::new(200, 0),
            account_to: "US333".to_string(),
            bic: None,
            date: "2024-03-01".to_string(),
            benef_name: "USD Recv".to_string(),
            benef_street: None,
            benef_city: None,
            benef_country: None,
            remittance_info1: None,
            remittance_info2: None,
            remittance_info3: None,
            remittance_info4: None,
            comment: None,
            payment_reason: "110".to_string(),
            details_of_charges: "SHA".to_string(),
        }));
        let xml = import.to_xml();
        let domestic_pos = xml.find("<DomesticTransaction>");
        let euro_pos = xml.find("<T2Transaction>");
        let foreign_pos = xml.find("<ForeignTransaction>");
        assert!(domestic_pos < euro_pos);
        assert!(euro_pos < foreign_pos);
    }
}
