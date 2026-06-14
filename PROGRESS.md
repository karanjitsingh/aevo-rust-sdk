# Aevo Rust SDK — Progress

Generating a Rust SDK for the [Aevo](https://aevo.xyz) exchange from the official
docs at <https://api-docs.aevo.xyz/llms.txt>.

## Architecture

```
aevo-rust-sdk/            (cargo workspace)
├── aevo-codegen/         generator (binary + library)
│   └── src/
│       ├── main.rs       CLI: download -> diff -> parse -> generate
│       ├── lib.rs        module wiring
│       ├── downloader.rs fetch llms.txt + every linked .md into specs/
│       ├── openapi.rs    serde model for the OpenAPI subset we consume
│       ├── parser.rs     extract OpenAPI JSON from markdown, build a unified spec
│       └── codegen.rs    emit Rust models + async client into aevo-sdk
├── aevo-sdk/             generated SDK (separate crate, must compile)
│   └── src/
│       ├── lib.rs        hand-written crate root
│       ├── error.rs      hand-written error type
│       ├── http.rs       hand-written AevoClient runtime + builder
│       ├── models.rs     GENERATED data models
│       └── endpoints.rs  GENERATED `impl AevoClient` methods
└── specs/                downloaded markdown docs (preserves doc paths)
```

## How it works

1. **Download** — `downloader` fetches `llms.txt`, extracts every `*.md` link,
   and downloads each into `specs/` preserving the URL path.
2. **Diff** — a SHA-256 manifest (`specs/.manifest.json`) records the hash of
   every downloaded file. Code generation only runs when content changed.
3. **Parse** — each markdown file embeds a fenced ```json``` OpenAPI 3.0 block.
   `parser` extracts it, deserializes via the `openapi` serde model, and merges
   all per-file specs into one unified spec (schemas + paths).
4. **Generate** — `codegen` emits `aevo-sdk/src/models.rs` (type aliases,
   enums, structs) and `aevo-sdk/src/endpoints.rs` (one async method per REST
   operation), then the SDK crate is built to verify it compiles.

## Task breakdown

- [x] 1. Scaffold workspace (`aevo-codegen` + `aevo-sdk`) and this file
- [x] 2. Downloader: llms.txt -> all `.md` into `specs/`
- [x] 3. OpenAPI parser (serde model + markdown JSON extraction + merge)
- [x] 4. Codegen: models + async client into `aevo-sdk`
- [x] 5. Diff-aware generation (SHA-256 manifest)
- [x] 6. Wire up `main` binary (download -> diff -> parse -> generate)
- [x] 7. Run generator; ensure `aevo-sdk` compiles; fix issues
- [x] 8. WebSocket client generation (channels + typed message structs + runtime)

## Results (latest run)

- **129** markdown docs downloaded into `specs/` (+ `.manifest.json`).
- **110** REST operations generated as async methods on `AevoClient`.
- **387** schemas generated as type aliases / enums / structs in `models.rs`.
- **7** WebSocket channels generated (`fills`, `orders`, `index`, `trades`,
  `book-ticker`, `orderbook-100ms`, `ticker-500ms`) with typed message structs
  plus a `Channel` enum in `ws/messages.rs`.
- `cargo build` (whole workspace) and `cargo test --doc -p aevo-sdk` both pass
  with **zero warnings**.
- Generation is **idempotent**: re-running with no doc changes skips
  generation; `--force` re-emits byte-identical output.

## Usage

```bash
# Download docs and regenerate the SDK only if the docs changed:
cargo run -p aevo-codegen

# Always regenerate (e.g. after changing the generator):
cargo run -p aevo-codegen -- --force

# Regenerate from already-downloaded specs/ without network access:
cargo run -p aevo-codegen -- --offline --force

# Verify the generated SDK compiles:
cargo build -p aevo-sdk
```

REST example:

```rust
use aevo_sdk::AevoClient;

let client = AevoClient::new();
let server_time = client.get_time().await?;            // public
let orderbook = client.get_orderbook("ETH-PERP".into()).await?; // query param

let authed = AevoClient::builder()
    .api_key("AEVO-KEY-VALUE")
    .api_secret("AEVO-SECRET-VALUE")
    .build()?;
let account = authed.get_account().await?;             // private (auth headers sent)
```

WebSocket example:

```rust
use aevo_sdk::ws::{WsClient, Channel};

let mut ws = WsClient::connect().await?;
ws.subscribe(&[Channel::Fills.as_str().to_string()]).await?;
while let Some(msg) = ws.next_message().await {
    let value = msg?; // raw serde_json::Value, or use next_typed::<FillsMessage>()
}
```

## Notes

- REST endpoint pages contain clean OpenAPI JSON and drive codegen.
- WebSocket / guide / changelog pages have no OpenAPI block. WS pages are parsed
  separately (their fenced JSON request/response examples) to infer message
  structs; other non-spec pages are downloaded for completeness but skipped.
- Parameter `$ref`s into `components.parameters` are resolved during parsing
  (this is why all 110 operations are captured rather than only the ~60 with
  inline parameters).
- The SDK uses `reqwest` (async, rustls) + `serde`; the WS client uses
  `tokio-tungstenite` (rustls). All request/response bodies are JSON.
