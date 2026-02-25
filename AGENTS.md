# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-25
**Commit:** a2e3285+
**Branch:** master

## OVERVIEW

Rust client library for the [Fio bank API](https://www.fio.cz/bankovni-sluzby/api-bankovnictvi) (Czech banking). Async HTTP via `reqwest`, JSON deserialization via `serde`, financial decimals via `rust_decimal`. Published as `fiocz-rs` v0.3.0 under MIT.

## STRUCTURE

```
fiocz-rs/
├── src/
│   ├── lib.rs                      # Fio struct + public API methods (thin orchestrator)
│   ├── client.rs                   # HTTP helpers (api_get/api_get_text/api_post) + status mapping
│   ├── validation.rs               # Date/year format validation (free functions)
│   ├── error.rs                    # Error enum (thiserror)
│   └── types/
│       ├── mod.rs                  # Re-exports
│       ├── account_statement.rs    # Statement/Info/TransactionList/TransactionData/LastStatementId
│       └── transaction/
│           ├── mod.rs              # DomesticTransaction/T2Transaction/ForeignTransaction structs + Type enum + Import + ImportBuilder
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
| Add HTTP helper | `src/client.rs` | Contains `impl Fio` block with `pub(crate)` HTTP methods + `map_status_error()` |
| Add validation | `src/validation.rs` | Free functions, `pub(crate)` — called from `lib.rs` |
| Add error variant | `src/error.rs` | Extend `Error` enum, add `#[error("...")]` |
| Add response type | `src/types/account_statement.rs` | Follow `#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]` + `#[serde(rename_all = "camelCase")]` |
| Add transaction type | `src/types/transaction/mod.rs` | Add concrete struct + variant to `Type` enum + builder method on `ImportBuilder` |
| Add XML conversion | `src/types/transaction/xml.rs` | Add `convert_*` free fn + match arm in `to_xml()` |
| Run example | `examples/workflow.rs` | `ACCESS_TOKEN=xxx cargo run --example workflow` |

## CONVENTIONS

- `#![forbid(unsafe_code)]` + `#![deny(missing_docs)]` — no unsafe, all public items documented
- `#[must_use]` on constructors and builder methods
- `#[serde(rename_all = "camelCase")]` on all API response structs
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields in transaction types
- All API methods return `Result<T, Error>` with the crate's custom error type
- Logging: `log` crate (`debug!`, `error!`, `warn!`) — never `println!` in library code
- HTTP status code mapping via `map_status_error()`: 409→`Limit`, 500→`Malformed`, 413→`TooLarge`
- Date format: `YYYY-MM-DD` validated by `validation::validate_date_string()`, year: `YYYY` by `validation::validate_year_string()`
- Type-safe builder: `ImportBuilder::domestic()` takes `DomesticTransaction`, not the `Type` enum
- Private modules: `client` and `validation` are `mod` (not `pub mod`) — internal only
- No custom rustfmt or clippy config — default formatting rules apply
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

## COMMANDS

```bash
cargo build                                    # Build library
cargo clippy -- -D warnings                    # Lint check (strict)
cargo test                                     # Run tests (31 unit + 1 doc)
cargo doc --open                               # Generate and view docs
ACCESS_TOKEN=xxx cargo run --example workflow   # Run example
```

## NOTES

- FIO API has rate limiting (HTTP 409) — the client maps this to `Error::Limit`
- Token is passed in URL path segments, not headers
- Import uses multipart form upload with XML body disguised as file (`import.xml`)
- `Cargo.lock` is gitignored (library convention)
- `.idea/` directory is present but gitignored
- `last_statement_id()` returns `LastStatementId` struct (not a tuple)
- CI runs on push/PR to master: check, clippy, test, doc, fmt
