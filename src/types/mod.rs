//! This module contains all the types used in the application.
pub mod account_statement;
pub mod import_response;
pub mod merchant;
pub mod transaction;

use std::fmt;

/// Supported export data formats
///
/// The FIO API supports multiple data formats for downloading movements
/// and statements. Not all formats are available for all endpoints.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// XML (Fio proprietary) format
    Xml,
    /// CSV format (semicolon-separated, UTF-8)
    Csv,
    /// GPC format (fixed-width, Windows-1250)
    Gpc,
    /// HTML format
    Html,
    /// OFX format
    Ofx,
    /// PDF format (statements only)
    Pdf,
    /// MT940 / STA format (statements only)
    Mt940,
    /// CBA XML / CAMT.053 Czech national format (statements only)
    CbaXml,
    /// SBA XML / CAMT.053 Slovak national format (statements only)
    SbaXml,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Xml => write!(f, "xml"),
            Self::Csv => write!(f, "csv"),
            Self::Gpc => write!(f, "gpc"),
            Self::Html => write!(f, "html"),
            Self::Ofx => write!(f, "ofx"),
            Self::Pdf => write!(f, "pdf"),
            Self::Mt940 => write!(f, "sta"),
            Self::CbaXml => write!(f, "cba_xml"),
            Self::SbaXml => write!(f, "sba_xml"),
        }
    }
}
