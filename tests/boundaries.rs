//! Checks the dependency rules between modules described in ARCHITECTURE.md:
//! `info` and `git` collect data and must not depend on the command line or on
//! how the data is displayed; `render` must not depend on the command line.
//!
//! Everything is one crate, so the compiler can't enforce this. The check
//! strips comments and strings, expands `use` trees, resolves `self`/`super`
//! from each file's module path, and inspects every `crate::` path and the
//! roots of external paths.

use std::fs;
use std::path::{Path, PathBuf};

struct Rule {
    dir: &'static str,
    forbidden_modules: &'static [&'static str],
    forbidden_crates: &'static [&'static str],
}

const RULES: [Rule; 4] = [
    Rule {
        dir: "src/info",
        forbidden_modules: &["cli", "options", "render"],
        forbidden_crates: &["clap", "owo_colors"],
    },
    Rule {
        dir: "src/git",
        forbidden_modules: &["cli", "options", "render", "info"],
        forbidden_crates: &["clap", "owo_colors"],
    },
    Rule {
        dir: "src/language",
        forbidden_modules: &["cli", "options", "render", "info"],
        forbidden_crates: &[],
    },
    Rule {
        dir: "src/render",
        forbidden_modules: &["cli", "options"],
        forbidden_crates: &[],
    },
];

#[test]
fn modules_respect_dependency_rules() {
    let mut violations = Vec::new();
    for rule in &RULES {
        let mut files = Vec::new();
        rust_files(Path::new(rule.dir), &mut files);
        assert!(!files.is_empty(), "no Rust files in {}", rule.dir);
        for file in files {
            let source = fs::read_to_string(&file).unwrap();
            for (line, path) in violations_in(&source, &module_path(&file), rule) {
                violations.push(format!("{}:{line}: {path}", file.display()));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "dependency rules from ARCHITECTURE.md are broken:\n{}",
        violations.join("\n")
    );
}

fn violations_in(source: &str, module: &[String], rule: &Rule) -> Vec<(usize, String)> {
    let code = strip_comments_and_strings(source);
    let mut found = Vec::new();
    for (offset, path) in paths(&code) {
        let line = code[..offset].matches('\n').count() + 1;
        let Some(resolved) = resolve(&path, module) else {
            continue;
        };
        let forbidden = match resolved.first().map(String::as_str) {
            Some("crate") => resolved
                .get(1)
                .is_some_and(|m| rule.forbidden_modules.contains(&m.as_str())),
            Some(root) => rule.forbidden_crates.contains(&root),
            None => false,
        };
        if forbidden {
            found.push((line, resolved.join("::")));
        }
    }
    found
}

/// The module path of a source file, e.g. `src/info/fields/size.rs` -> `[info, fields, size]`.
fn module_path(file: &Path) -> Vec<String> {
    let relative = file.strip_prefix("src").unwrap().with_extension("");
    let mut segments: Vec<String> = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    if segments.last().is_some_and(|s| s == "mod" || s == "lib") {
        segments.pop();
    }
    segments
}

/// Makes `self::`/`super::` paths absolute (`crate::...`). Other paths are
/// returned as is: their first segment is an external crate or a local name.
fn resolve(path: &[String], module: &[String]) -> Option<Vec<String>> {
    let mut segments = path.iter().peekable();
    let mut resolved: Vec<String> = match segments.peek().map(|s| s.as_str()) {
        Some("crate") => return Some(path.to_vec()),
        Some("self") => {
            segments.next();
            std::iter::once("crate".to_string())
                .chain(module.iter().cloned())
                .collect()
        }
        Some("super") => std::iter::once("crate".to_string())
            .chain(module.iter().cloned())
            .collect(),
        Some(_) => return Some(path.to_vec()),
        None => return None,
    };
    for segment in segments {
        if segment == "super" {
            if resolved.len() <= 1 {
                return None;
            }
            resolved.pop();
        } else {
            resolved.push(segment.clone());
        }
    }
    Some(resolved)
}

/// Every path in the code with at least two segments (`a::b`), with `use`
/// trees expanded. Returns the byte offset where each path starts.
fn paths(code: &str) -> Vec<(usize, Vec<String>)> {
    let mut result = Vec::new();
    let mut rest = code;
    let mut offset = 0;
    while let Some(start) = find_word(rest, "use") {
        let (before, after) = rest.split_at(start);
        result.extend(inline_paths(before, offset));
        let end = after.find(';').map_or(after.len(), |i| i + 1);
        let tree = normalize_use_tree(after[3..end].trim_end_matches(';'));
        for path in expand_use_tree(&tree) {
            result.push((offset + start, path));
        }
        offset += start + end;
        rest = &after[end..];
    }
    result.extend(inline_paths(rest, offset));
    result
}

fn find_word(haystack: &str, word: &str) -> Option<usize> {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    haystack.match_indices(word).map(|(i, _)| i).find(|&i| {
        let before = haystack[..i].chars().next_back();
        let after = haystack[i + word.len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

/// Paths written in code, e.g. `crate::render::foo()` or `clap::ValueEnum`.
fn inline_paths(code: &str, offset: usize) -> Vec<(usize, Vec<String>)> {
    let bytes = code.as_bytes();
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut result = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if !is_ident(bytes[i]) || (i > 0 && (is_ident(bytes[i - 1]) || bytes[i - 1] == b':')) {
            i += 1;
            continue;
        }
        let start = i;
        let mut segments = Vec::new();
        loop {
            let word_start = i;
            while i < bytes.len() && is_ident(bytes[i]) {
                i += 1;
            }
            segments.push(code[word_start..i].to_string());
            let mut j = i;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if code[j..].starts_with("::") {
                j += 2;
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j < bytes.len() && is_ident(bytes[j]) {
                    i = j;
                    continue;
                }
            }
            break;
        }
        if segments.len() > 1 {
            result.push((offset + start, segments));
        }
        i = i.max(start + 1);
    }
    result
}

/// Collapses whitespace to single spaces and removes it around `::`, braces
/// and commas, so only the spaces of `x as y` remain.
fn normalize_use_tree(tree: &str) -> String {
    let collapsed = tree.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = collapsed;
    for token in ["::", "{", "}", ","] {
        out = out
            .split(token)
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(token);
    }
    out
}

/// Expands `a::{b::c, d as e}` into `[a, b, c]` and `[a, d]`.
fn expand_use_tree(tree: &str) -> Vec<Vec<String>> {
    let tree = tree.trim_start_matches("::");
    if let Some(brace) = tree.find('{') {
        let prefix: Vec<String> = tree[..brace]
            .split("::")
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        let inner = &tree[brace + 1..tree.rfind('}').unwrap_or(tree.len())];
        split_top_level(inner)
            .into_iter()
            .flat_map(expand_use_tree)
            .map(|suffix| prefix.iter().cloned().chain(suffix).collect())
            .collect()
    } else {
        let path = tree.split(" as ").next().unwrap_or(tree);
        vec![
            path.split("::")
                .filter(|s| !s.is_empty() && *s != "*")
                .map(String::from)
                .collect(),
        ]
    }
}

fn split_top_level(list: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    for (i, c) in list.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&list[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&list[start..]);
    parts.into_iter().filter(|p| !p.is_empty()).collect()
}

/// Replaces comments and string/char literals with spaces, keeping newlines so
/// byte offsets still map to the right line.
fn strip_comments_and_strings(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let blank = |c: char| if c == '\n' { '\n' } else { ' ' };
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            while i < chars.len() && chars[i] != '\n' {
                out.push(' ');
                i += 1;
            }
        } else if c == '/' && next == Some('*') {
            let mut depth = 0;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    out.push_str("  ");
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    out.push_str("  ");
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    out.push(blank(chars[i]));
                    i += 1;
                }
            }
        } else if c == 'r' && (next == Some('#') || next == Some('"')) {
            let hashes = chars[i + 1..].iter().take_while(|&&c| c == '#').count();
            if chars.get(i + 1 + hashes) != Some(&'"') {
                out.push(c);
                i += 1;
                continue;
            }
            let closing: String = std::iter::once('"')
                .chain(std::iter::repeat_n('#', hashes))
                .collect();
            i += 2 + hashes;
            out.push_str(&" ".repeat(2 + hashes));
            while i < chars.len() && !chars[i..].iter().collect::<String>().starts_with(&closing) {
                out.push(blank(chars[i]));
                i += 1;
            }
            out.push_str(&" ".repeat(closing.len()));
            i += closing.len();
        } else if c == '"' {
            out.push(' ');
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' {
                    out.push(' ');
                    i += 1;
                }
                if i < chars.len() {
                    out.push(blank(chars[i]));
                    i += 1;
                }
            }
            out.push(' ');
            i += 1;
        } else if c == '\'' && is_char_literal(&chars[i..]) {
            out.push(' ');
            i += 1;
            while i < chars.len() && chars[i] != '\'' {
                if chars[i] == '\\' {
                    out.push(' ');
                    i += 1;
                }
                out.push(' ');
                i += 1;
            }
            out.push(' ');
            i += 1;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// `'a'` or `'\n'` (a char literal) as opposed to `'a` (a lifetime).
fn is_char_literal(chars: &[char]) -> bool {
    match chars.get(1) {
        Some('\\') => true,
        Some(_) => chars.get(2) == Some(&'\''),
        None => false,
    }
}

fn rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod checker {
    use super::*;

    fn check(source: &str) -> Vec<String> {
        let module = vec!["info".to_string(), "fields".to_string(), "size".to_string()];
        violations_in(source, &module, &RULES[0])
            .into_iter()
            .map(|(_, path)| path)
            .collect()
    }

    #[test]
    fn flags_forbidden_paths() {
        assert_eq!(
            check("use crate::render::text::Line;"),
            ["crate::render::text::Line"]
        );
        assert_eq!(
            check("use crate::{info::x, render::format::DisplayOptions};"),
            ["crate::render::format::DisplayOptions"]
        );
        assert_eq!(
            check("use super::super::super::render::text;"),
            ["crate::render::text"]
        );
        assert_eq!(check("fn f() { crate::cli::Cli::default(); }").len(), 1);
        assert_eq!(check("#[derive(clap::ValueEnum)] enum E {}").len(), 1);
        assert_eq!(check("use owo_colors::DynColors;").len(), 1);
        assert_eq!(
            check("use crate::render::{\n    format::DisplayOptions as D,\n    text,\n};"),
            [
                "crate::render::format::DisplayOptions",
                "crate::render::text"
            ]
        );
    }

    #[test]
    fn ignores_comments_strings_and_allowed_paths() {
        assert!(check("// uses DisplayOptions from crate::render").is_empty());
        assert!(check("/* crate::render */ let s = \"crate::cli\";").is_empty());
        assert!(check("let s = r#\"use crate::render;\"#;").is_empty());
        assert!(check("use super::super::manifest::Manifest;").is_empty());
        assert!(check("use self::context::Context as RepoContext;").is_empty());
        assert!(check("use crate::info::{dates::serialize_time, has_assets};").is_empty());
        assert!(check("use crate::git::metrics::GitMetrics; fn f<'a>(x: &'a str) {}").is_empty());
    }
}
