pub mod error;
pub mod types;

use reqwest::StatusCode;
use serde::{de::DeserializeOwned};
use log::{debug, error};
use crate::error::FioError;
use crate::types::account_statement::Statement;
use crate::types::transaction::Import;

pub struct Fio {
    token: String,
}

impl Fio {
    pub fn new(token: &str) -> Self {
        Fio { token: token.to_string() }
    }

    async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, FioError> {
        match self.api_get_text(rest_method).await {
            Ok(v) => {
                let de: Result<T, _> = serde_json::from_str(&v);
                match de {
                    Ok(reply) => Ok(reply),
                    Err(e) => {
                        error!("Couldn't parse reply for {} call: {}", rest_method, e);
                        debug!("Source JSON: {}", v);
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn api_get_text(&self, rest_method: &str) -> Result<String, FioError> {
        match reqwest::get(format!("https://www.fio.cz/ib_api/rest/{}", rest_method)).await {
            Ok(resp) => {
                println!("{:?}", resp.status());
                if resp.status() == StatusCode::CONFLICT {
                    return Err(FioError::Limit);
                }
                if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(FioError::Malformed);
                }
                if resp.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    return Err(FioError::TooLarge);
                }
                match resp.text().await {
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(e) => {
                        Err(e.into())
                    }
                }
            }
            Err(e) => { Err(e.into()) }
        }
    }

    async fn api_post(&self, rest_method: &str, body: String) -> Result<String, FioError> {
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new().text("token", self.token.clone()).text("type", "xml").part("file", match reqwest::multipart::Part::text(body).file_name("import.xml").mime_str("application/xml") {
            Ok(file) => { file }
            Err(e) => {
                return Err(e.into());
            }
        });
        match client.post(format!("https://www.fio.cz/ib_api/rest/{}", rest_method)).multipart(form).send().await {
            Ok(resp) => {
                println!("{:?}", resp.status());
                if resp.status() == StatusCode::CONFLICT {
                    return Err(FioError::Limit);
                }
                if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(FioError::Malformed);
                }
                if resp.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    return Err(FioError::TooLarge);
                }
                println!("Status: {}", resp.status());
                match resp.text().await {
                    Ok(v) => {
                        Ok(v)
                    }
                    Err(e) => {
                        Err(e.into())
                    }
                }
            }
            Err(e) => { Err(e.into()) }
        }
    }

    async fn api_get_empty(&self, rest_method: &str) -> Result<(), FioError> {
        match self.api_get_text(rest_method).await {
            Ok(_) => { Ok(()) }
            Err(e) => {
                Err(e)
            }
        }
    }

    fn validate_date_string(date: &str) -> bool {
        if date.len() != 10 {
            println!("Incorrect length");
            return false;
        }
        for (index, c) in date.chars().enumerate() {
            if [0i32, 1i32, 2i32, 3i32, 5i32, 6i32, 8i32, 9i32].contains(&(index as i32)) {
                if !c.is_digit(10) {
                    println!("{c} is not a digit on position {index}");
                    return false;
                }
            } else {
                if c != '-' {
                    println!("{c} is not a dash on position {index}");
                    return false;
                }
            }
        }
        true
    }

    fn validate_year_string(year: &str) -> bool {
        if year.len() != 4 {
            println!("Incorrect length");
            return false;
        }
        for (index, c) in year.chars().enumerate() {
            if !c.is_digit(10) {
                println!("{c} is not a digit on position {index}");
                return false;
            }
        }
        true
    }

    pub async fn movements_in_period(&self, start: &str, end: &str) -> Result<Statement, FioError> {
        if !Self::validate_date_string(start) {
            return Err(FioError::InvalidDateFormat);
        }
        if !Self::validate_date_string(end) {
            return Err(FioError::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!("periods/{token}/{start}/{end}/transactions.json", token = self.token)).await
    }

    pub async fn movements_since_last(&self) -> Result<Statement, FioError> {
        self.api_get::<Statement>(&format!("last/{token}/transactions.json", token = self.token)).await
    }

    pub async fn statements(&self, year: &str, id: &str) -> Result<Statement, FioError> {
        if !Self::validate_year_string(year) {
            return Err(FioError::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!("by-id/{token}/{year}/{id}/transactions.json", token = self.token)).await
    }

    pub async fn set_last_id(&self, id: &str) -> Result<(), FioError> {
        self.api_get_empty(&format!("set-last-id/{token}/{id}/", token = self.token)).await
    }

    pub async fn set_last_date(&self, date: &str) -> Result<(), FioError> {
        if !Self::validate_date_string(date) {
            return Err(FioError::InvalidDateFormat);
        }
        self.api_get_empty(&format!("set-last-date/{token}/{date}/", token = self.token)).await
    }

    pub async fn last_statement_id(&self) -> Result<(String, String), FioError> {
        match self.api_get_text(&format!("lastStatement/{token}/statement", token = self.token)).await {
            Ok(id) => {
                match id.split_once(',') {
                    None => {
                        Err(FioError::InvalidResponse("Not enough elements returned".to_string()))
                    }
                    Some(result) => {
                        Ok((result.0.to_string(), result.1.to_string()))
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn import_transactions(&self, transactions: Import) -> Result<String, FioError> {
        match self.api_post("import/", transactions.to_xml()).await {
            Ok(v) => {
                Ok(v)
            }
            Err(e) => { Err(e) }
        }
    }
}