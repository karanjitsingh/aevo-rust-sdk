//! Downloads the Aevo documentation index (`llms.txt`) and every markdown
//! file it links to, preserving the URL path under a local `specs/` directory.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};

/// Base of the documentation site.
pub const DOCS_BASE: &str = "https://api-docs.aevo.xyz";
/// The documentation index file.
pub const LLMS_TXT_URL: &str = "https://api-docs.aevo.xyz/llms.txt";

/// A single downloaded document.
#[derive(Debug, Clone)]
pub struct Doc {
    /// URL path relative to the docs base, e.g. `reference/postorders.md`.
    pub rel_path: String,
    /// Absolute URL the document was fetched from.
    pub url: String,
    /// Raw markdown contents.
    pub contents: String,
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()
}

fn get_text(agent: &ureq::Agent, url: &str) -> Result<String> {
    let body = agent
        .get(url)
        .call()
        .with_context(|| format!("GET {url}"))?
        .into_string()
        .with_context(|| format!("reading body of {url}"))?;
    Ok(body)
}

/// Extract every absolute `*.md` link from the `llms.txt` index.
///
/// Links appear inside markdown as `(https://api-docs.aevo.xyz/....md)`. We
/// scan for the `https://` scheme and stop at the first `.md`, which is robust
/// against the index being a single very long line.
pub fn parse_md_links(llms_txt: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let bytes = llms_txt.as_bytes();
    let needle = "https://";
    let mut search_from = 0usize;
    while let Some(rel) = llms_txt[search_from..].find(needle) {
        let start = search_from + rel;
        // Find the end of the URL: first whitespace, ')' or markdown control char.
        let mut end = start;
        while end < bytes.len() {
            let c = bytes[end] as char;
            if c.is_whitespace() || c == ')' || c == '(' || c == ']' || c == '[' || c == '>' {
                break;
            }
            end += 1;
        }
        let url = &llms_txt[start..end];
        if url.ends_with(".md") && url.starts_with(DOCS_BASE) && seen.insert(url.to_string()) {
            links.push(url.to_string());
        }
        search_from = end.max(start + 1);
    }
    links
}

/// Convert an absolute docs URL into a relative path under `specs/`.
fn url_to_rel_path(url: &str) -> String {
    url.strip_prefix(DOCS_BASE)
        .unwrap_or(url)
        .trim_start_matches('/')
        .to_string()
}

/// Download `llms.txt` and all linked markdown files.
///
/// Returns the list of downloaded docs (including `llms.txt` itself, stored as
/// `llms.txt`). Files are written under `specs_dir`, preserving their paths.
pub fn download_all(specs_dir: &Path) -> Result<Vec<Doc>> {
    let agent = agent();
    let mut docs = Vec::new();

    // 1. Index file.
    let index = get_text(&agent, LLMS_TXT_URL).context("downloading llms.txt")?;
    docs.push(Doc {
        rel_path: "llms.txt".to_string(),
        url: LLMS_TXT_URL.to_string(),
        contents: index.clone(),
    });

    // 2. Every linked markdown file.
    let links = parse_md_links(&index);
    eprintln!("Found {} markdown links in llms.txt", links.len());
    for (i, url) in links.iter().enumerate() {
        let rel_path = url_to_rel_path(url);
        match get_text(&agent, url) {
            Ok(contents) => {
                eprintln!("  [{:>3}/{}] {}", i + 1, links.len(), rel_path);
                docs.push(Doc {
                    rel_path,
                    url: url.clone(),
                    contents,
                });
            }
            Err(e) => {
                eprintln!("  [{:>3}/{}] FAILED {rel_path}: {e:#}", i + 1, links.len());
            }
        }
    }

    write_docs(specs_dir, &docs)?;
    Ok(docs)
}

/// Write all docs to disk under `specs_dir`, preserving relative paths.
fn write_docs(specs_dir: &Path, docs: &[Doc]) -> Result<()> {
    for doc in docs {
        let path = specs_dir.join(&doc.rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        fs::write(&path, &doc.contents)
            .with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}

/// Load previously downloaded docs from disk (used to compare / regenerate
/// without re-downloading). Returns docs keyed by relative path.
pub fn load_docs_from_disk(specs_dir: &Path) -> Result<Vec<Doc>> {
    let mut docs = Vec::new();
    load_recursive(specs_dir, specs_dir, &mut docs)?;
    docs.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    Ok(docs)
}

fn load_recursive(root: &Path, dir: &Path, out: &mut Vec<Doc>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            load_recursive(root, &path, out)?;
        } else if path.extension().map(|e| e == "md" || e == "txt").unwrap_or(false) {
            let rel_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let url = format!("{DOCS_BASE}/{rel_path}");
            out.push(Doc {
                rel_path,
                url,
                contents,
            });
        }
    }
    Ok(())
}

/// A SHA-256 content manifest: relative path -> hex digest.
pub type Manifest = BTreeMap<String, String>;

/// Compute the SHA-256 manifest for a set of docs.
pub fn compute_manifest(docs: &[Doc]) -> Manifest {
    use sha2::{Digest, Sha256};
    docs.iter()
        .map(|d| {
            let mut hasher = Sha256::new();
            hasher.update(d.contents.as_bytes());
            (d.rel_path.clone(), format!("{:x}", hasher.finalize()))
        })
        .collect()
}

/// Path to the manifest file inside `specs_dir`.
pub fn manifest_path(specs_dir: &Path) -> PathBuf {
    specs_dir.join(".manifest.json")
}

/// Load the previous manifest, if any.
pub fn load_manifest(specs_dir: &Path) -> Result<Option<Manifest>> {
    let path = manifest_path(specs_dir);
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let manifest: Manifest = serde_json::from_str(&text)
        .with_context(|| format!("parsing {}", path.display()))?;
    Ok(Some(manifest))
}

/// Persist the manifest to disk.
pub fn save_manifest(specs_dir: &Path, manifest: &Manifest) -> Result<()> {
    let path = manifest_path(specs_dir);
    let text = serde_json::to_string_pretty(manifest)?;
    fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
