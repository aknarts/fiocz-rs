pub mod error;
pub mod types;

use serde::{de::DeserializeOwned};
use crate::error::FioError;
use crate::types::account_statement::Statement;

pub struct Fio {
    token: String,
}

impl Fio {
    pub fn new(token: &str) -> Self {
        Fio { token: token.to_string() }
    }

    async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, FioError> {
        match reqwest::get(format!("https://www.fio.cz/ib_api/rest/{}", rest_method)).await {
            Ok(resp) => {
                match resp.json::<T>().await {
                    Ok(v) => { Ok(v) }
                    Err(e) => { Err(e.into()) }
                }
            }
            Err(e) => { Err(e.into()) }
        }
    }

    pub async fn get_movements_in_period(&self, start: &str, end: &str) -> Result<Statement, FioError> {
        self.api_get::<Statement>(&format!("periods/{token}/{start}/{end}/transactions.json", token = self.token)).await
    }
}