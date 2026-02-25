# fiocz-rs

[![CI](https://github.com/aknarts/fiocz-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/aknarts/fiocz-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/fiocz-rs.svg)](https://crates.io/crates/fiocz-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Async Rust client for the [Fio banka API](https://www.fio.cz/bankovni-sluzby/api-bankovnictvi).

Provides typed access to account movements, statements, transaction imports, and merchant card data. Handles rate limiting (30s between requests), multipart file uploads, and multiple export formats.

## Features

- **Account movements** — by date range or since last download, as parsed JSON or raw format (CSV, XML, OFX, GPC, HTML)
- **Official statements** — by year/ID, with support for PDF, MT940, and CAMT.053 formats
- **Transaction import** — domestic, euro (T2), and foreign payments via type-safe builder with compile-time validation
- **Raw import** — ABO, SEPA pain.001, and SEPA pain.008 format support
- **Merchant transactions** — POS terminal and payment gateway data
- **Bookmarks** — set download cursor by movement ID or date
- **Rate limiting** — automatic 30-second delay between API calls per token
- **Type-safe enums** — `DetailsOfCharges`, `DomesticPaymentType`, `EuroPaymentType`, `ExportFormat`

## Quick start

Add to `Cargo.toml`:

```toml
[dependencies]
fiocz-rs = "0.5"
tokio = { version = "1", features = ["full"] }
```

```rust
use fiocz_rs::Fio;

#[tokio::main]
async fn main() {
    let fio = Fio::new("your-api-token");

    match fio.movements_in_period("2025-01-01", "2025-01-31").await {
        Ok(statement) => {
            let info = &statement.account_statement.info;
            let txns = &statement.account_statement.transaction_list.transaction;
            println!("{} ({}) — {} transactions",
                info.account_id, info.currency, txns.len());
        }
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
```

## API coverage

| Endpoint | Method | Format support |
|----------|--------|----------------|
| Movements in period | `movements_in_period` / `movements_in_period_raw` | JSON, XML, CSV, GPC, HTML, OFX |
| Movements since last | `movements_since_last` / `movements_since_last_raw` | JSON, XML, CSV, GPC, HTML, OFX |
| Official statements | `statements` / `statements_raw` | JSON, XML, CSV, PDF, MT940, CAMT.053 |
| Last statement ID | `last_statement_id` | Text |
| Set bookmark (ID) | `set_last_id` | — |
| Set bookmark (date) | `set_last_date` | — |
| Import (Fio XML) | `import_transactions` | XML (type-safe builder) |
| Import (raw) | `import_raw` | ABO, pain.001, pain.008 |
| Merchant transactions | `merchant_transactions_raw` | XML only |

## Transaction import

Build payment orders with compile-time type safety:

```rust
use fiocz_rs::types::transaction::{DomesticTransaction, Import};
use rust_decimal::Decimal;

let import = Import::builder()
    .domestic(DomesticTransaction {
        account_from: "2101179627".to_string(),
        currency: "CZK".to_string(),
        amount: Decimal::new(10000, 2), // 100.00 CZK
        account_to: "1234567890".to_string(),
        bank_code: "0800".to_string(),
        ks: None,
        vs: Some("1234567890".to_string()),
        ss: None,
        date: "2025-06-01".to_string(),
        message_for_recipient: Some("Invoice 42".to_string()),
        comment: None,
        payment_reason: None,
        payment_type: None,
    })
    .build();
```

Foreign transactions enforce mandatory fields at compile time (BIC, address, remittance info).

## Examples

Each example reads the API token from `ACCESS_TOKEN` env var:

```bash
# Account movements (JSON)
ACCESS_TOKEN=xxx cargo run --example movements_in_period -- 2025-01-01 2025-01-31

# Account movements (CSV)
ACCESS_TOKEN=xxx cargo run --example movements_in_period -- 2025-01-01 2025-01-31 csv

# Movements since last download
ACCESS_TOKEN=xxx cargo run --example movements_since_last

# Official statement
ACCESS_TOKEN=xxx cargo run --example statements -- 2025 1

# Last statement number
ACCESS_TOKEN=xxx cargo run --example last_statement_id

# Set download bookmark
ACCESS_TOKEN=xxx cargo run --example set_bookmark -- date 2025-01-15
ACCESS_TOKEN=xxx cargo run --example set_bookmark -- id 24230217199

# Import domestic payment
ACCESS_TOKEN=xxx cargo run --example import

# Merchant card transactions
ACCESS_TOKEN=xxx cargo run --example merchant -- 2025-01-01 2025-01-31
```

## Rate limiting

The FIO API allows one request per 30 seconds per token. The client enforces this automatically — if you make requests faster, it sleeps until the interval has passed. The rate limiter is shared across clones of the same `Fio` instance.

## Error handling

All API methods return `Result<T, fiocz_rs::error::Error>`:

| HTTP status | Error variant | Meaning |
|-------------|--------------|---------|
| 404 | `Token` | Invalid or deactivated token |
| 409 | `Limit` | Rate limit exceeded |
| 413 | `TooLarge` | Response too large |
| 422 | `HistoricalDataLocked` | Data older than 90 days (requires auth unlock in internet banking) |
| 500 | `Malformed` | Malformed request |

## License

MIT
