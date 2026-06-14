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
- [ ] 2. Downloader: llms.txt -> all `.md` into `specs/`
- [ ] 3. OpenAPI parser (serde model + markdown JSON extraction + merge)
- [ ] 4. Codegen: models + async client into `aevo-sdk`
- [ ] 5. Diff-aware generation (SHA-256 manifest)
- [ ] 6. Wire up `main` binary (download -> diff -> parse -> generate)
- [ ] 7. Run generator; ensure `aevo-sdk` compiles; fix issues

## Notes

- REST endpoint pages contain clean OpenAPI JSON and drive codegen.
- WebSocket / guide / changelog pages are downloaded for completeness but have
  no OpenAPI JSON block, so they are skipped during code generation.
- The SDK uses `reqwest` (async, rustls) + `serde`. All response/request bodies
  in Aevo are JSON.
