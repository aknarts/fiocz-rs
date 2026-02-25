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
//! * Get account movements in period (JSON or raw format)
//! * Get account movements since last
//! * Get account statement
//! * Get last statement id
//! * Set last movement id
//! * Set last movement date
//! * Import transactions (Fio XML)
//! * Get merchant card transactions
//!

mod client;
pub mod error;
pub mod types;
mod validation;

use crate::error::Error;
use crate::types::account_statement::{LastStatementId, Statement};
use crate::types::transaction::Import;
use crate::types::ExportFormat;

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

    /// Get account movements in period (JSON)
    /// # Arguments
    /// * `start` - Start date in format YYYY-MM-DD
    /// * `end` - End date in format YYYY-MM-DD
    /// # Returns
    /// * `Statement` - Account movements
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    /// * `Error::Limit` - Too many requests
    /// * `Error::HistoricalDataLocked` - Data older than 90 days requires auth
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

    /// Get account movements in period in specified format (raw text)
    /// # Arguments
    /// * `start` - Start date in format YYYY-MM-DD
    /// * `end` - End date in format YYYY-MM-DD
    /// * `format` - Export format
    /// # Returns
    /// * `String` - Raw response body in the requested format
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    /// * `Error::Limit` - Too many requests
    /// * `Error::HistoricalDataLocked` - Data older than 90 days requires auth
    pub async fn movements_in_period_raw(
        &self,
        start: &str,
        end: &str,
        format: ExportFormat,
    ) -> Result<String, Error> {
        if !validation::validate_date_string(start) {
            return Err(Error::InvalidDateFormat);
        }
        if !validation::validate_date_string(end) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get_text(&format!(
            "periods/{token}/{start}/{end}/transactions.{format}",
            token = self.token
        ))
        .await
    }

    /// Get account movements since last download (JSON)
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

    /// Get account movements since last download in specified format (raw text)
    /// # Arguments
    /// * `format` - Export format
    /// # Returns
    /// * `String` - Raw response body in the requested format
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn movements_since_last_raw(&self, format: ExportFormat) -> Result<String, Error> {
        self.api_get_text(&format!(
            "last/{token}/transactions.{format}",
            token = self.token
        ))
        .await
    }

    /// Get account statement (JSON)
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

    /// Get account statement in specified format (raw text)
    /// # Arguments
    /// * `year` - Year in format YYYY
    /// * `id` - Statement ID
    /// * `format` - Export format
    /// # Returns
    /// * `String` - Raw response body in the requested format
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    pub async fn statements_raw(
        &self,
        year: &str,
        id: &str,
        format: ExportFormat,
    ) -> Result<String, Error> {
        if !validation::validate_year_string(year) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get_text(&format!(
            "by-id/{token}/{year}/{id}/transactions.{format}",
            token = self.token
        ))
        .await
    }

    /// Set last movement id (bookmark)
    /// # Arguments
    /// * `id` - Movement ID
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn set_last_id(&self, id: &str) -> Result<(), Error> {
        self.api_get_empty(&format!("set-last-id/{token}/{id}/", token = self.token))
            .await
    }

    /// Set last movement date (bookmark)
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

    /// Import transactions using Fio XML format
    /// # Arguments
    /// * `transactions` - Transactions to import
    /// # Returns
    /// * `String` - Raw XML response from the bank
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn import_transactions(&self, transactions: Import) -> Result<String, Error> {
        self.api_post("import/", "xml", transactions.to_xml()).await
    }

    /// Import transactions using a raw payload in the specified format
    ///
    /// Supports formats: `xml` (Fio XML), `abo` (ABO/Czech domestic),
    /// `pain001_xml` (SEPA Credit Transfer), `pain008_xml` (SEPA Direct Debit)
    /// # Arguments
    /// * `format` - Import format type string (`xml`, `abo`, `pain001_xml`, `pain008_xml`)
    /// * `body` - Raw payload string
    /// # Returns
    /// * `String` - Raw XML response from the bank
    /// # Errors
    /// * `Error::Limit` - Too many requests
    pub async fn import_raw(&self, format: &str, body: String) -> Result<String, Error> {
        self.api_post("import/", format, body).await
    }

    /// Get merchant card transactions for a period
    ///
    /// Returns POS terminal and payment gateway transactions.
    /// Note: only XML format is supported by the API for this endpoint.
    /// # Arguments
    /// * `start` - Start date in format YYYY-MM-DD
    /// * `end` - End date in format YYYY-MM-DD
    /// # Returns
    /// * `String` - Raw XML response (merchant endpoint only supports XML)
    /// # Errors
    /// * `Error::InvalidDateFormat` - Invalid date format
    /// * `Error::Limit` - Too many requests
    pub async fn merchant_transactions_raw(&self, start: &str, end: &str) -> Result<String, Error> {
        if !validation::validate_date_string(start) {
            return Err(Error::InvalidDateFormat);
        }
        if !validation::validate_date_string(end) {
            return Err(Error::InvalidDateFormat);
        }
        self.api_get_text(&format!(
            "merchant/{token}/{start}/{end}/transactions.xml",
            token = self.token
        ))
        .await
    }
}
