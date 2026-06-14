//! Extracts embedded OpenAPI JSON from the downloaded markdown files and merges
//! every per-file document into a single unified spec for code generation.

use std::collections::BTreeMap;

use anyhow::Result;

use crate::downloader::Doc;
use crate::openapi::{OpenApi, Operation, PathItem, Schema};

/// A unified view across all documentation pages.
#[derive(Debug, Default)]
pub struct UnifiedSpec {
    /// All component schemas, keyed by their original (snake_case) name.
    pub schemas: BTreeMap<String, Schema>,
    /// Every REST operation discovered, in deterministic order.
    pub operations: Vec<EndpointOp>,
}

/// A single REST operation with enough context to generate a client method.
#[derive(Debug, Clone)]
pub struct EndpointOp {
    /// URL path template, e.g. `/orders/{order_id}`.
    pub path: String,
    /// Uppercase HTTP method, e.g. `GET`.
    pub method: String,
    /// The operation definition.
    pub op: Operation,
    /// Source document relative path (for diagnostics / comments).
    pub source: String,
}

/// Extract every fenced ```json block from a markdown string.
fn extract_json_blocks(md: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut rest = md;
    while let Some(start) = rest.find("```json") {
        let after = &rest[start + "```json".len()..];
        if let Some(end) = after.find("```") {
            blocks.push(after[..end].trim().to_string());
            rest = &after[end + 3..];
        } else {
            break;
        }
    }
    blocks
}

/// Parse the OpenAPI definition embedded in a single markdown document.
///
/// A page may contain several ```json blocks (request/response examples plus
/// the spec). We pick the block that deserializes into an OpenAPI document with
/// at least one path — that is the real definition.
pub fn parse_openapi_from_markdown(md: &str) -> Option<OpenApi> {
    let mut best: Option<OpenApi> = None;
    for block in extract_json_blocks(md) {
        if let Ok(doc) = serde_json::from_str::<OpenApi>(&block) {
            if !doc.paths.is_empty() {
                return Some(doc);
            }
            // Keep a components-only doc as a fallback.
            if best.is_none() && !doc.components.schemas.is_empty() {
                best = Some(doc);
            }
        }
    }
    best
}

/// Build a unified spec from all downloaded docs.
pub fn build_unified_spec(docs: &[Doc]) -> Result<UnifiedSpec> {
    let mut spec = UnifiedSpec::default();
    let mut skipped = 0usize;

    // Sort docs by path for deterministic output.
    let mut docs_sorted: Vec<&Doc> = docs.iter().collect();
    docs_sorted.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));

    for doc in docs_sorted {
        let Some(openapi) = parse_openapi_from_markdown(&doc.contents) else {
            skipped += 1;
            continue;
        };

        // Merge schemas. On name collision, keep the first definition seen but
        // prefer a "richer" one (more properties / enum values) if it appears.
        for (name, schema) in openapi.components.schemas {
            spec.schemas
                .entry(name)
                .and_modify(|existing| {
                    if schema_richness(&schema) > schema_richness(existing) {
                        *existing = schema.clone();
                    }
                })
                .or_insert(schema);
        }

        // Collect operations.
        for (path, item) in &openapi.paths {
            collect_operations(path, item, &doc.rel_path, &mut spec.operations);
        }
    }

    // Deterministic operation order: by path, then method.
    spec.operations
        .sort_by(|a, b| (a.path.as_str(), a.method.as_str()).cmp(&(b.path.as_str(), b.method.as_str())));
    // De-duplicate identical (method, path) pairs that appear in multiple docs.
    spec.operations.dedup_by(|a, b| a.path == b.path && a.method == b.method);

    eprintln!(
        "Parsed {} operations and {} schemas ({} non-spec pages skipped)",
        spec.operations.len(),
        spec.schemas.len(),
        skipped
    );
    Ok(spec)
}

fn collect_operations(path: &str, item: &PathItem, source: &str, out: &mut Vec<EndpointOp>) {
    for (method, op) in item.operations() {
        out.push(EndpointOp {
            path: path.to_string(),
            method: method.to_string(),
            op: op.clone(),
            source: source.to_string(),
        });
    }
}

/// A rough "richness" score so that when the same schema name appears in
/// multiple docs we keep the most descriptive definition.
fn schema_richness(s: &Schema) -> usize {
    s.properties.len() * 10 + s.enum_values.len() * 5 + s.required.len()
        + s.description.as_ref().map(|d| d.len().min(50)).unwrap_or(0)
}
