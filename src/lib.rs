#![forbid(unsafe_code)]
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

mod client;
pub mod error;
pub mod types;
mod validation;

use crate::error::Error;
use crate::types::account_statement::{LastStatementId, Statement};
use crate::types::transaction::Import;

/// Fiocz API client
#[derive(Clone)]
pub struct Fio {
    token: String,
}

impl Fio {
    /// Create new API client
    /// # Arguments
    /// * `token` - Fio API token
    #[must_use]
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
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
        if !validation::validate_date_string(start) {
            return Err(Error::InvalidDateFormat);
        }
        if !validation::validate_date_string(end) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!(
            "periods/{token}/{start}/{end}/transactions.json",
            token = self.token
        ))
        .await
    }
    /// Get account movements since last
    /// # Returns
    /// * `Statement` - Account movements
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn movements_since_last(&self) -> Result<Statement, Error> {
        self.api_get::<Statement>(&format!(
            "last/{token}/transactions.json",
            token = self.token
        ))
        .await
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
        if !validation::validate_year_string(year) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get::<Statement>(&format!(
            "by-id/{token}/{year}/{id}/transactions.json",
            token = self.token
        ))
        .await
    }

    /// Set last movement id
    /// # Arguments
    /// * `id` - Movement ID
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn set_last_id(&self, id: &str) -> Result<(), Error> {
        self.api_get_empty(&format!("set-last-id/{token}/{id}/", token = self.token))
            .await
    }

    /// Set last movement date
    /// # Arguments
    /// * `date` - Date in format YYYY-MM-DD
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    pub async fn set_last_date(&self, date: &str) -> Result<(), Error> {
        if !validation::validate_date_string(date) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get_empty(&format!(
            "set-last-date/{token}/{date}/",
            token = self.token
        ))
        .await
    }

    /// Get last statement id
    /// # Returns
    /// * `LastStatementId` - Year and statement ID
    /// # Errors
    /// * `Error::InvalidResponse` - Invalid response
    /// * `Error::Limit` - Too many requests
    pub async fn last_statement_id(&self) -> Result<LastStatementId, Error> {
        match self
            .api_get_text(&format!(
                "lastStatement/{token}/statement",
                token = self.token
            ))
            .await
        {
            Ok(id) => id.split_once(',').map_or_else(
                || {
                    Err(Error::InvalidResponse(
                        "Not enough elements returned".to_string(),
                    ))
                },
                |result| {
                    Ok(LastStatementId {
                        year: result.0.to_string(),
                        id: result.1.to_string(),
                    })
                },
            ),
            Err(e) => Err(e),
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
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}
