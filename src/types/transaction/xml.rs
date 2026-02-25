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
