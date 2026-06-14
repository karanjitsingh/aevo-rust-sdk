//! Identifier-naming helpers shared by the REST and WebSocket generators.

/// Rust reserved keywords that must be escaped when used as identifiers.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Escape a snake_case field/variable name so it is a valid Rust identifier.
pub fn escape_ident(name: &str) -> String {
    let cleaned = sanitize(name);
    let cleaned = if cleaned.is_empty() {
        "field".to_string()
    } else if cleaned.chars().next().unwrap().is_ascii_digit() {
        format!("_{cleaned}")
    } else {
        cleaned
    };
    if RUST_KEYWORDS.contains(&cleaned.as_str()) {
        format!("r#{cleaned}")
    } else {
        cleaned
    }
}

/// Replace any non-alphanumeric character with `_`.
fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    // Collapse repeated underscores.
    let mut collapsed = String::with_capacity(out.len());
    let mut prev_underscore = false;
    for c in out.chars() {
        if c == '_' {
            if !prev_underscore {
                collapsed.push(c);
            }
            prev_underscore = true;
        } else {
            collapsed.push(c);
            prev_underscore = false;
        }
    }
    collapsed.trim_matches('_').to_string()
}

/// Convert an arbitrary name to `snake_case`.
pub fn to_snake_case(name: &str) -> String {
    let sanitized = sanitize(name);
    let mut out = String::with_capacity(sanitized.len() + 4);
    let chars: Vec<char> = sanitized.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev_lower_or_digit = i > 0 && (chars[i - 1].is_ascii_lowercase() || chars[i - 1].is_ascii_digit());
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_ascii_lowercase();
            if i > 0 && (prev_lower_or_digit || next_lower) && !out.ends_with('_') {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    let mut collapsed = String::with_capacity(out.len());
    let mut prev_underscore = false;
    for c in out.chars() {
        if c == '_' {
            if !prev_underscore {
                collapsed.push(c);
            }
            prev_underscore = true;
        } else {
            collapsed.push(c);
            prev_underscore = false;
        }
    }
    collapsed.trim_matches('_').to_string()
}

/// Convert an arbitrary name to `PascalCase`.
pub fn to_pascal_case(name: &str) -> String {
    let snake = to_snake_case(name);
    let mut out = String::with_capacity(snake.len());
    for part in snake.split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    if out.is_empty() {
        out.push_str("Unnamed");
    }
    if out.chars().next().unwrap().is_ascii_digit() {
        out.insert(0, '_');
    }
    out
}

/// Escape a snake_case method name for `fn` definitions.
pub fn escape_method_name(name: &str) -> String {
    escape_ident(&to_snake_case(name))
}
