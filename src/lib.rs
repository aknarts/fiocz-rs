#![deny(missing_docs)]

//! Fio API client
//!
//! # Example
//!
//! ```no_run
//! use fiocz_rs::Fio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let fio = Fio::new("token");
//!
//!    match fio.movements_since_last().await {
//!       Ok(statement) => {
//!           println!("Got newest movements containing {} transactions", statement.account_statement.transaction_list.transaction.len())
//!      }
//!      Err(e) => {
//!          println!("Failed to get newest account movements: {:?}", e)
//!      }
//!  }
//! }
//! ```
//!
//! # Current functionality
//!
//! * Get account movements in period
//! * Get account movements since last
//! * Get account statement
//! * Get last statement id
//! * Set last movement id
//! * Set last movement date
//! * Import transactions
//!


pub mod error;
pub mod types;

use reqwest::StatusCode;
use serde::{de::DeserializeOwned};
use log::{debug, error, warn};
use crate::error::Error;
use crate::types::account_statement::Statement;
use crate::types::transaction::Import;

/// Fiocz API client
pub struct Fio {
    token: String,
}

impl Fio {
    /// Create new API client
    /// # Arguments
    /// * `token` - Fio API token
    #[must_use]
    pub fn new(token: &str) -> Self {
        Self { token: token.to_string() }
    }

    async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
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

    async fn api_get_text(&self, rest_method: &str) -> Result<String, Error> {
        match reqwest::get(format!("https://www.fio.cz/ib_api/rest/{rest_method}")).await {
            Ok(resp) => {
                if resp.status() == StatusCode::CONFLICT {
                    return Err(Error::Limit);
                }
                if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(Error::Malformed);
                }
                if resp.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    return Err(Error::TooLarge);
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

    async fn api_post(&self, rest_method: &str, body: String) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new().text("token", self.token.clone()).text("type", "xml").part("file", match reqwest::multipart::Part::text(body).file_name("import.xml").mime_str("application/xml") {
            Ok(file) => { file }
            Err(e) => {
                return Err(e.into());
            }
        });
        match client.post(format!("https://www.fio.cz/ib_api/rest/{rest_method}")).multipart(form).send().await {
            Ok(resp) => {
                if resp.status() == StatusCode::CONFLICT {
                    return Err(Error::Limit);
                }
                if resp.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    return Err(Error::Malformed);
                }
                if resp.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    return Err(Error::TooLarge);
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

    async fn api_get_empty(&self, rest_method: &str) -> Result<(), Error> {
        match self.api_get_text(rest_method).await {
            Ok(_) => { Ok(()) }
            Err(e) => {
                Err(e)
            }
        }
    }

    fn validate_date_string(date: &str) -> bool {
        if date.len() != 10 {
            error!("Incorrect length");
            return false;
        }
        for (index, c) in date.chars().enumerate() {
            if [0usize, 1usize, 2usize, 3usize, 5usize, 6usize, 8usize, 9usize].contains(&index) {
                if !c.is_ascii_digit() {
                    warn!("{c} is not a digit on position {index}");
                    return false;
                }
            } else if c != '-' {
                warn!("{c} is not a dash on position {index}");
                return false;
            }
        }
        true
    }

    fn validate_year_string(year: &str) -> bool {
        if year.len() != 4 {
            error!("Incorrect length");
            return false;
        }
        for (index, c) in year.chars().enumerate() {
            if !c.is_ascii_digit() {
                warn!("{c} is not a digit on position {index}");
                return false;
            }
        }
        true
    }

    /// Get account movements in period
    /// # Arguments
    /// * `start` - Start date in format YYYY-MM-DD
    /// * `end` - End date in format YYYY-MM-DD
    /// # Returns
    /// * `Statement` - Account movements
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    /// * `Error::Limit` - Too many requests
    pub async fn movements_in_period(&self, start: &str, end: &str) -> Result<Statement, Error> {
        if !Self::validate_date_string(start) {
            return Err(Error::InvalidDateFormat);
        }
        if !Self::validate_date_string(end) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!("periods/{token}/{start}/{end}/transactions.json", token = self.token)).await
    }
    /// Get account movements since last
    /// # Returns
    /// * `Statement` - Account movements
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn movements_since_last(&self) -> Result<Statement, Error> {
        self.api_get::<Statement>(&format!("last/{token}/transactions.json", token = self.token)).await
    }

    /// Get account statement
    /// # Arguments
    /// * `year` - Year in format YYYY
    /// * `id` - Statement ID
    /// # Returns
    /// * `Statement` - Account statement
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    pub async fn statements(&self, year: &str, id: &str) -> Result<Statement, Error> {
        if !Self::validate_year_string(year) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!("by-id/{token}/{year}/{id}/transactions.json", token = self.token)).await
    }

    /// Set last movement id
    /// # Arguments
    /// * `id` - Movement ID
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn set_last_id(&self, id: &str) -> Result<(), Error> {
        self.api_get_empty(&format!("set-last-id/{token}/{id}/", token = self.token)).await
    }

    /// Set last movement date
    /// # Arguments
    /// * `date` - Date in format YYYY-MM-DD
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    pub async fn set_last_date(&self, date: &str) -> Result<(), Error> {
        if !Self::validate_date_string(date) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get_empty(&format!("set-last-date/{token}/{date}/", token = self.token)).await
    }

    /// Get last statement id
    /// # Returns
    /// * `String` - Year
    /// * `String` - ID
    /// # Errors
    /// * `Error::InvalidResponse` - Invalid response
    /// * `Error::Limit` - Too many requests
    pub async fn last_statement_id(&self) -> Result<(String, String), Error> {
        match self.api_get_text(&format!("lastStatement/{token}/statement", token = self.token)).await {
            Ok(id) => {
                id.split_once(',').map_or_else(|| Err(Error::InvalidResponse("Not enough elements returned".to_string())),
                                               |result| Ok((result.0.to_string(), result.1.to_string())))
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Import transactions
    /// # Arguments
    /// * `transactions` - Transactions to import
    /// # Returns
    /// * `String` - Import ID
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn import_transactions(&self, transactions: Import) -> Result<String, Error> {
        match self.api_post("import/", transactions.to_xml()).await {
            Ok(v) => {
                Ok(v)
            }
            Err(e) => { Err(e) }
        }
    }
}