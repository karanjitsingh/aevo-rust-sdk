# aevo-rust-sdk

A Rust SDK for the [Aevo](https://aevo.xyz) decentralized derivatives exchange,
**generated** from Aevo's official API documentation
(<https://api-docs.aevo.xyz/llms.txt>).

The repository is a Cargo workspace with two crates:

| Crate          | Role                                                                          |
| -------------- | ----------------------------------------------------------------------------- |
| `aevo-sdk`     | The SDK you depend on. Async REST client + WebSocket client. **Generated.**   |
| `aevo-codegen` | The generator. Downloads the docs and emits `aevo-sdk`'s models and methods.  |

> The model and endpoint files in `aevo-sdk` are produced by `aevo-codegen`.
> Don't hand-edit `models.rs`, `endpoints.rs`, or `ws/messages.rs` — re-run the
> generator instead.

## Layout

```
aevo-rust-sdk/
├── aevo-codegen/         generator (binary + library)
│   └── src/
│       ├── main.rs       CLI: download -> diff -> parse -> generate
│       ├── downloader.rs fetch llms.txt + every linked .md into specs/
│       ├── openapi.rs    serde model for the OpenAPI subset we consume
│       ├── parser.rs     extract OpenAPI JSON from markdown, build a unified spec
│       ├── wsgen.rs      parse WebSocket pages, infer message structs
│       ├── codegen.rs    emit Rust models + async REST client
│       └── names.rs      identifier-naming helpers
├── aevo-sdk/             the SDK (separate crate, must compile)
│   └── src/
│       ├── lib.rs        crate root            (hand-written)
│       ├── error.rs      error type            (hand-written)
│       ├── http.rs       AevoClient + builder  (hand-written)
│       ├── models.rs     data models           (GENERATED)
│       ├── endpoints.rs  REST methods          (GENERATED)
│       └── ws/
│           ├── mod.rs      WsClient runtime    (hand-written)
│           └── messages.rs channels + payloads (GENERATED)
├── specs/                downloaded markdown docs + .manifest.json
└── PROGRESS.md           design notes + task tracking
```

## How generation works

1. **Download** — `aevo-codegen` fetches `llms.txt`, extracts every `*.md` link,
   and downloads each into `specs/`, preserving the documentation path.
2. **Diff** — a SHA-256 manifest (`specs/.manifest.json`) records the hash of
   each file. Generation only runs when content changed (override with `--force`).
3. **Parse** — each REST page embeds a fenced ` ```json ` OpenAPI 3.0 block.
   The parser extracts it, resolves `components.parameters` `$ref`s, and merges
   all pages into one unified spec (schemas + operations).
4. **Generate** — `codegen` emits `models.rs` (type aliases, enums, structs) and
   `endpoints.rs` (one async method per REST operation); `wsgen` emits
   `ws/messages.rs` (typed message structs + a `Channel` enum).

WebSocket pages are human-formatted tables rather than OpenAPI, so their typed
structs are inferred from the embedded request/response JSON examples. Guide and
changelog pages are downloaded for completeness but contain no spec and are
skipped during code generation.

## Regenerating the SDK

```bash
# Download docs and regenerate only if the docs changed:
cargo run -p aevo-codegen

# Always regenerate (e.g. after changing the generator):
cargo run -p aevo-codegen -- --force

# Regenerate from already-downloaded specs/ without network access:
cargo run -p aevo-codegen -- --offline --force

# Verify the generated SDK compiles:
cargo build -p aevo-sdk
```

Current output: **110** REST operations, **387** schemas, and **7** WebSocket
channels. The whole workspace builds with zero warnings.

## Using the SDK

Add the crate to your `Cargo.toml` (path or git dependency):

```toml
[dependencies]
aevo-sdk = { path = "aevo-sdk" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### REST — public endpoints

```rust
use aevo_sdk::AevoClient;

#[tokio::main]
async fn main() -> Result<(), aevo_sdk::Error> {
    let client = AevoClient::new(); // defaults to https://api.aevo.xyz

    let time = client.get_time().await?;
    let orderbook = client.get_orderbook("ETH-PERP".to_string()).await?;
    let markets = client.get_markets(Some("ETH".to_string()), None).await?;

    println!("{time:?}\n{orderbook:?}\n{} markets", markets.len());
    Ok(())
}
```

### REST — private (authenticated) endpoints

Authenticated endpoints send the `AEVO-KEY` / `AEVO-SECRET` headers. Provide them
via the builder:

```rust
use aevo_sdk::AevoClient;

let client = AevoClient::builder()
    .api_key("YOUR-AEVO-KEY")
    .api_secret("YOUR-AEVO-SECRET")
    .build()?;

let account = client.get_account().await?;
let positions = client.get_positions().await?;
```

> Note: order-placement endpoints expect a pre-computed EIP-712 `signature` and
> `salt` in the request payload. The SDK transmits whatever you supply; it does
> not compute signatures for you.

### Testnet

```rust
use aevo_sdk::{AevoClient, TESTNET_BASE_URL};

let client = AevoClient::builder().base_url(TESTNET_BASE_URL).build()?;
```

### WebSocket

```rust
use aevo_sdk::ws::{WsClient, Channel};
use aevo_sdk::ws::messages::FillsMessage;

#[tokio::main]
async fn main() -> Result<(), aevo_sdk::Error> {
    let mut ws = WsClient::connect().await?; // wss://ws.aevo.xyz
    ws.subscribe(&[Channel::Fills.as_str().to_string()]).await?;

    while let Some(msg) = ws.next_message().await {
        let value = msg?;            // raw serde_json::Value
        println!("{value}");
        // Or deserialize into a generated struct:
        // let fill: FillsMessage = serde_json::from_value(value)?;
    }
    Ok(())
}
```

`WsClient` also exposes `unsubscribe`, `auth(key, secret)`, `send_json`, and
`next_typed::<T>()` for typed deserialization. The `Channel` enum lists the
known channels (`Fills`, `Orders`, `Index`, `Trades`, `BookTicker`,
`Orderbook100ms`, `Ticker500ms`) plus `Other(String)` for anything else.

## Dependencies

- `aevo-sdk`: `reqwest` (async, rustls), `serde`/`serde_json`, `thiserror`,
  `tokio-tungstenite` (rustls) for WebSockets.
- `aevo-codegen`: `ureq` (download), `serde_json`, `sha2`, `anyhow`.

## Status & limitations

- Generation is reproducible/idempotent: with no doc changes it skips work;
  `--force` re-emits byte-identical output.
- A couple of WebSocket pages (`ticker-500ms`, `index`) have unstructured table
  layouts; their inferred typed structs may not perfectly match the live payload.
  Use `WsClient::next_message()` (raw JSON) when in doubt.
- The SDK has not been exercised against the live API in this repo (compile and
  doctest verified only).

## License

Apache-2.0.
