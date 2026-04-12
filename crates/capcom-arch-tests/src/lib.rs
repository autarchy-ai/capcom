//! Architecture tests for the capcom workspace.
//!
//! Integration tests in `tests/` enforce workspace-level invariants via
//! `cargo test`. This module exposes shared helpers used by those tests:
//! a recursive source-file walker and a per-line matcher that understands
//! Rust comments and `adr-override:` escape annotations.

use std::fs;
use std::path::{Path, PathBuf};

/// Returns the absolute path to the workspace root.
///
/// Derives from the compile-time `CARGO_MANIFEST_DIR` of this crate
/// (`crates/capcom-arch-tests`) by walking up two parents.
///
/// # Panics
///
/// Panics if this crate is somehow not under `crates/<name>/` at the
/// workspace root. The architecture-tests crate is hard-coded to live at
/// `crates/capcom-arch-tests/` and any move must update this function in
/// the same change.
#[must_use]
pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("capcom-arch-tests should live under crates/")
        .parent()
        .expect("crates/ should live under the workspace root")
        .to_path_buf()
}

/// Collects every `.rs` file and every `Cargo.toml` under `crates/`,
/// excluding the architecture-tests crate itself (which contains banned
/// patterns as literal strings by design).
///
/// # Panics
///
/// Panics if the workspace's `crates/` directory is missing, unreadable,
/// or contains unreadable entries. Both conditions indicate a broken
/// checkout and should fail the test suite loudly rather than silently
/// returning an empty result.
#[must_use]
pub fn collect_workspace_sources() -> Vec<PathBuf> {
    let crates_dir = workspace_root().join("crates");
    let mut files = Vec::new();
    let entries = fs::read_dir(&crates_dir).expect("workspace has a crates/ directory");
    for entry in entries {
        let entry = entry.expect("readable crates/ entry");
        let crate_path = entry.path();
        let Some(crate_name) = crate_path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if crate_name == "capcom-arch-tests" {
            continue;
        }
        let cargo_toml = crate_path.join("Cargo.toml");
        if cargo_toml.is_file() {
            files.push(cargo_toml);
        }
        let src_dir = crate_path.join("src");
        if src_dir.is_dir() {
            collect_rust_files(&src_dir, &mut files);
        }
        let tests_dir = crate_path.join("tests");
        if tests_dir.is_dir() {
            collect_rust_files(&tests_dir, &mut files);
        }
    }
    files
}

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {}: {e}", dir.display()));
    for entry in entries {
        let entry =
            entry.unwrap_or_else(|e| panic!("failed to read entry in {}: {e}", dir.display()));
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

/// Returns `true` if the line is a Rust comment line: `//`, `///`, `//!`,
/// `/*`, or a block-comment continuation (`*` at start). TOML comments
/// (`#` at start) are also treated as comment lines.
#[must_use]
pub fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("//")
        || trimmed.starts_with("/*")
        || trimmed.starts_with("*/")
        || trimmed.starts_with("* ")
        || trimmed == "*"
        || trimmed.starts_with('#')
}

/// Returns `true` if the line carries an `adr-override:` annotation that
/// names any of the given ADR IDs (`ADR-NNN`).
#[must_use]
pub fn has_override(line: &str, adrs: &[&str]) -> bool {
    line.contains("adr-override:") && adrs.iter().any(|adr| line.contains(adr))
}

/// Formats `path` as a workspace-relative string if possible, falling back
/// to the absolute representation.
#[must_use]
pub fn workspace_relative(path: &Path) -> String {
    let root = workspace_root();
    path.strip_prefix(&root)
        .map_or_else(|_| path.display().to_string(), |p| p.display().to_string())
}
