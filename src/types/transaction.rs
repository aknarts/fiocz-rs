use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Import {
    pub orders: Vec<TransactionType>,
}

impl Import {
    pub fn new() -> Self {
        Import { orders: vec![] }
    }
    pub fn builder() -> ImportBuilder {
        ImportBuilder::new()
    }
}

pub struct ImportBuilder {
    domestic: Vec<TransactionType>,
    euro: Vec<TransactionType>,
    foreign: Vec<TransactionType>,
}

impl ImportBuilder {
    pub fn new() -> Self {
        return Self {
            domestic: vec![],
            euro: vec![],
            foreign: vec![],
        };
    }

    pub fn domestic(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::DomesticTransaction { .. } = transaction {
            self.domestic.push(transaction);
        }
        self
    }

    pub fn euro(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::T2Transaction { .. } = transaction {
            self.euro.push(transaction);
        }
        self
    }

    pub fn foreign(&mut self, transaction: TransactionType) -> &mut ImportBuilder {
        if let TransactionType::ForeignTransaction { .. } = transaction {
            self.foreign.push(transaction);
        }
        self
    }

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TransactionType {
    #[serde(rename_all = "camelCase")]
    DomesticTransaction {
        account_from: String,
        currency: String,
        amount: f64,
        account_to: String,
        bank_code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        ks: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vs: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ss: Option<String>,
        date: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        message_for_recipient: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_type: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    T2Transaction {
        account_from: String,
        currency: String,
        amount: f64,
        account_to: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bic: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ks: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vs: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ss: Option<String>,
        date: String,
        benef_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info1: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        payment_type: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    ForeignTransaction {
        account_from: String,
        currency: String,
        amount: f64,
        account_to: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bic: Option<String>,
        date: String,
        benef_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_street: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_city: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        benef_country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info1: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remittance_info4: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        payment_reason: String,
        details_of_charges: String,
    },
}

#[derive(Serialize, )]
pub struct Test {
    pub val: Vec<TestEnum>,
}

#[derive(Serialize, )]
pub enum TestEnum {
    #[serde(rename_all = "camelCase")]
    FirstEnum {
        content: String
    },
    #[serde(rename_all = "camelCase")]
    SecondEnum {
        cont: String
    },
}