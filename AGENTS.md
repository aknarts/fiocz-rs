# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-25
**Commit:** 89467ba+
**Branch:** master

## OVERVIEW

Rust client library for the [Fio bank API](https://www.fio.cz/bankovni-sluzby/api-bankovnictvi) (Czech banking). Async HTTP via `reqwest`, JSON deserialization via `serde`, financial decimals via `rust_decimal`. Built-in 30-second rate limiting per API spec. Published as `fiocz-rs` v0.5.0 under MIT.

## STRUCTURE

```
fiocz-rs/
├── src/
│   ├── lib.rs                      # Fio struct + public API methods (thin orchestrator) + rate limiter
│   ├── client.rs                   # HTTP helpers (api_get/api_get_text/api_post) + status mapping + rate limit enforcement
│   ├── validation.rs               # Date/year format validation (free functions)
│   ├── error.rs                    # Error enum (thiserror) — Limit/Token/Malformed/TooLarge/HistoricalDataLocked
│   └── types/
│       ├── mod.rs                  # Re-exports + ExportFormat enum
│       ├── account_statement.rs    # Statement/Info/TransactionList/TransactionData/LastStatementId
│       ├── import_response.rs      # ImportResponse struct for parsing import results
│       ├── merchant.rs             # MerchantStatement/MerchantTransaction for POS/gateway data
│       └── transaction/
│           ├── mod.rs              # DomesticTransaction/T2Transaction/ForeignTransaction + Type enum + Import + ImportBuilder + payment enums
│           └── xml.rs              # XML generation (to_xml + convert_domestic/euro/foreign)
├── examples/
│   └── workflow.rs                 # Usage demo (requires ACCESS_TOKEN env var)
├── .github/workflows/ci.yml       # CI: check + clippy + test + doc + fmt
├── Cargo.toml
└── README.md
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add API method | `src/lib.rs` | Add pub async method on `Fio`, use helpers from `client.rs` |
| Add HTTP helper | `src/client.rs` | Contains `impl Fio` block with `pub(crate)` HTTP methods + `map_status_error()` + rate limit |
| Add validation | `src/validation.rs` | Free functions, `pub(crate)` — called from `lib.rs` |
| Add error variant | `src/error.rs` | Extend `Error` enum, add `#[error("...")]`, update `map_status_error()` in client.rs |
| Add response type | `src/types/account_statement.rs` | Follow `#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]` + `#[serde(rename_all = "camelCase")]` |
| Add transaction type | `src/types/transaction/mod.rs` | Add concrete struct + variant to `Type` enum + builder method on `ImportBuilder` |
| Add XML conversion | `src/types/transaction/xml.rs` | Add `convert_*` free fn + match arm in `to_xml()` |
| Add export format | `src/types/mod.rs` | Extend `ExportFormat` enum + `Display` impl |
| Run example | `examples/workflow.rs` | `ACCESS_TOKEN=xxx cargo run --example workflow` |

## CONVENTIONS

- `#![forbid(unsafe_code)]` + `#![deny(missing_docs)]` — no unsafe, all public items documented
- `#[must_use]` on constructors and builder methods
- `#[serde(rename_all = "camelCase")]` on all API response structs
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields in transaction types
- All API methods return `Result<T, Error>` with the crate's custom error type
- Logging: `log` crate (`debug!`, `error!`, `warn!`) — never `println!` in library code
- HTTP status code mapping via `map_status_error()`: 404→`Token`, 409→`Limit`, 413→`TooLarge`, 422→`HistoricalDataLocked`, 500→`Malformed`
- Date format: `YYYY-MM-DD` validated by `validation::validate_date_string()`, year: `YYYY` by `validation::validate_year_string()`
- Type-safe builder: `ImportBuilder::domestic()` takes `DomesticTransaction`, not the `Type` enum
- Type-safe enums: `DetailsOfCharges`, `DomesticPaymentType`, `EuroPaymentType` with `Display` impls
- Private modules: `client` and `validation` are `mod` (not `pub mod`) — internal only
- Rate limiting: 30s minimum between requests, enforced automatically via `Arc<Mutex<Instant>>`
- API base URL hardcoded: `https://fioapi.fio.cz/v1/rest/`

## ANTI-PATTERNS (THIS PROJECT)

- **No `unsafe`** — enforced by `#![forbid(unsafe_code)]`. Compilation fails if added.
- **No `unwrap()`/`expect()` in library code** — only in `examples/`. Use `?` or explicit error handling.
- **No `as any` / type suppression** — all types explicit.

## UNIQUE STYLES

- **Manual XML generation** in `transaction/xml.rs` (`to_xml()`) — builds XML via `Vec<String>` concatenation instead of an XML crate. Each transaction type has its own `convert_*` free function.
- **HashMap-based transactions** in `account_statement.rs` — `Vec<HashMap<String, Option<TransactionData>>>` for flexible field mapping from API.
- **Untagged enum** `TransactionDataEnum` — handles polymorphic JSON values (int/string/decimal) from the API.
- **Type-safe builder** for `Import` — `ImportBuilder` with `domestic(DomesticTransaction)`, `euro(T2Transaction)`, `foreign(ForeignTransaction)` methods enforcing correct types at compile time.
- **Split `impl Fio`** — public API in `lib.rs`, HTTP internals in `client.rs` via separate `impl Fio` block.
- **Dual API methods** — JSON-parsed (`movements_in_period`) + raw text (`movements_in_period_raw`) variants for format flexibility.

## COMMANDS

```bash
cargo build                                    # Build library
cargo clippy -- -D warnings                    # Lint check (strict)
cargo test                                     # Run tests (34 unit + 1 doc)
cargo doc --open                               # Generate and view docs
ACCESS_TOKEN=xxx cargo run --example workflow   # Run example
```

## NOTES

- FIO API has rate limiting (HTTP 409) — client auto-sleeps 30s between requests
- Token is passed in URL path segments, not headers
- Import uses multipart form upload with XML body disguised as file (`import.xml`)
- `Cargo.lock` is gitignored (library convention)
- `last_statement_id()` returns `LastStatementId` struct (not a tuple)
- Merchant endpoint only supports XML format (not JSON)
- ForeignTransaction has mandatory fields (bic, address, remittanceInfo1) per API v1.9
- CI runs on push/PR to master: check, clippy, test, doc, fmt
