//! `aevo-codegen` — downloads the Aevo API docs and regenerates the `aevo-sdk`
//! crate when the downloaded content changes.
//!
//! Usage:
//! ```text
//! cargo run -p aevo-codegen            # download + regenerate on diff
//! cargo run -p aevo-codegen -- --force # always regenerate
//! cargo run -p aevo-codegen -- --offline # skip download, use specs/ on disk
//! ```

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use aevo_codegen::codegen;
use aevo_codegen::downloader::{self, Doc};
use aevo_codegen::parser;
use aevo_codegen::wsgen;

struct Options {
    force: bool,
    offline: bool,
}

fn parse_args() -> Options {
    let mut force = false;
    let mut offline = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--force" => force = true,
            "--offline" => offline = true,
            other => eprintln!("warning: ignoring unknown argument {other:?}"),
        }
    }
    Options { force, offline }
}

/// Locate the workspace root (the directory containing this crate's parent).
fn workspace_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points at aevo-codegen/; the workspace is its parent.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or(manifest)
}

fn main() -> Result<()> {
    let opts = parse_args();
    let root = workspace_root();
    let specs_dir = root.join("specs");
    let sdk_src = root.join("aevo-sdk").join("src");

    std::fs::create_dir_all(&specs_dir)
        .with_context(|| format!("creating {}", specs_dir.display()))?;

    // 1. Acquire docs (download unless --offline).
    let docs: Vec<Doc> = if opts.offline {
        eprintln!("Offline mode: loading docs from {}", specs_dir.display());
        downloader::load_docs_from_disk(&specs_dir)?
    } else {
        eprintln!("Downloading docs from {}", downloader::LLMS_TXT_URL);
        downloader::download_all(&specs_dir)?
    };
    if docs.is_empty() {
        anyhow::bail!("no documentation found (specs dir empty and no download)");
    }

    // 2. Diff against the previous manifest.
    let new_manifest = downloader::compute_manifest(&docs);
    let prev_manifest = downloader::load_manifest(&specs_dir)?;
    let changed = match &prev_manifest {
        Some(prev) => prev != &new_manifest,
        None => true,
    };

    if !changed && !opts.force {
        eprintln!("No content changes detected; skipping generation. Use --force to override.");
        return Ok(());
    }
    if opts.force {
        eprintln!("Forcing regeneration (--force).");
    } else {
        report_diff(prev_manifest.as_ref(), &new_manifest);
    }

    // 3. Parse the OpenAPI specs into a unified model.
    let spec = parser::build_unified_spec(&docs)?;

    // 4. Generate REST models + endpoints.
    let rest = codegen::generate(&spec);
    write_if_changed(&sdk_src.join("models.rs"), &rest.models)?;
    write_if_changed(&sdk_src.join("endpoints.rs"), &rest.endpoints)?;

    // 5. Generate the WebSocket message types.
    let ws_messages = wsgen::generate(&docs);
    write_if_changed(&sdk_src.join("ws").join("messages.rs"), &ws_messages)?;

    // 6. Persist the manifest so subsequent runs can diff.
    downloader::save_manifest(&specs_dir, &new_manifest)?;

    eprintln!("Generation complete.");
    eprintln!("  models.rs       <- {} schemas", spec.schemas.len());
    eprintln!("  endpoints.rs    <- {} REST operations", spec.operations.len());
    eprintln!("  ws/messages.rs  <- WebSocket channels");
    Ok(())
}

/// Write a file only if its contents differ, to keep mtimes/diffs minimal.
fn write_if_changed(path: &Path, contents: &str) -> Result<()> {
    if let Ok(existing) = std::fs::read_to_string(path) {
        if existing == contents {
            eprintln!("  unchanged: {}", path.display());
            return Ok(());
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, contents).with_context(|| format!("writing {}", path.display()))?;
    eprintln!("  wrote: {}", path.display());
    Ok(())
}

/// Print a short summary of which docs changed.
fn report_diff(prev: Option<&downloader::Manifest>, new: &downloader::Manifest) {
    let Some(prev) = prev else {
        eprintln!("No previous manifest; generating from scratch ({} files).", new.len());
        return;
    };
    let mut added = 0;
    let mut modified = 0;
    for (path, hash) in new {
        match prev.get(path) {
            None => added += 1,
            Some(old) if old != hash => modified += 1,
            _ => {}
        }
    }
    let removed = prev.keys().filter(|k| !new.contains_key(*k)).count();
    eprintln!("Changes detected: {added} added, {modified} modified, {removed} removed.");
}
