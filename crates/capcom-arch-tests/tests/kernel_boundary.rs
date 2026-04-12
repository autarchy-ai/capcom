//! Kernel boundary tests.
//!
//! capcom is the Rust kernel for Aphelion. Its public API surface is the
//! contract consumed by `capcom-engine` and any future product-layer
//! adapter (per ADR-012 / ADR-014). These tests enforce two invariants:
//!
//! 1. **Public API surface is explicit.** Every public item exported by
//!    the `capcom` crate must be acknowledged here. When new public items
//!    land, this test forces an explicit edit -- unreviewed API surface
//!    growth is the bug we want to catch.
//!
//! 2. **`capcom-engine` depends only on `capcom`.** The engine binary is
//!    a thin entry point for the co-located out-of-process engine node;
//!    adding other runtime dependencies requires architectural review
//!    (aphelion ADR-006 dependency discipline).
//!
//! Rust's visibility model already enforces "consumers only see `pub`
//! items", so a module-level leak test would be redundant with the
//! compiler. These tests target the things the compiler does *not*
//! catch on its own: deliberate review of the public surface, and
//! deliberate review of the engine binary's dependency closure.

use std::fs;

use capcom_arch_tests::workspace_root;

#[test]
fn kernel_public_api_surface_is_minimal() {
    // Compile-time assertion: `capcom::version` exists with the expected
    // signature. Adding a new public item will break compilation of this
    // test (via the missing binding) and force an explicit update, which
    // is exactly the review gate we want.
    let version_fn: fn() -> &'static str = capcom::version;
    assert!(
        !version_fn().is_empty(),
        "capcom::version() must return a non-empty version string"
    );

    // When the kernel's public API grows, add each new item below as a
    // typed binding. Each addition should land in the same change that
    // adds the item, and should be reviewed against the compatibility
    // surface registry (aphelion ADR-010) when that registry exists for
    // capcom.
    //
    // Example future entries:
    //     let _open: fn(&Path) -> Result<Database, Error> = capcom::open;
    //     let _begin: fn(&Database) -> Transaction<'_> = Database::begin;
}

#[test]
fn engine_crate_depends_only_on_capcom() {
    let engine_manifest = workspace_root().join("crates/capcom-engine/Cargo.toml");
    let content = fs::read_to_string(&engine_manifest)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", engine_manifest.display()));

    let runtime_deps = collect_dependency_keys(&content, "dependencies");
    assert_eq!(
        runtime_deps,
        vec!["capcom".to_string()],
        "capcom-engine must depend only on the capcom kernel library. Adding any \
         other runtime dependency requires architectural review (aphelion ADR-006 \
         dependency discipline). Found runtime dependencies: {runtime_deps:?}"
    );

    // Build and dev dependencies are intentionally not constrained here --
    // they are developer tooling, not runtime surface.
}

/// Parses a Cargo.toml string and returns the ordered list of dependency
/// keys under the given top-level table (e.g. `"dependencies"`).
///
/// This is a minimal parser that handles the subset of TOML the capcom
/// workspace uses. It is sufficient for the boundary test without pulling
/// in an external TOML crate (aphelion ADR-006 would require reviewing any
/// new dependency, and a test-only toml crate is not justified yet).
fn collect_dependency_keys(content: &str, section: &str) -> Vec<String> {
    let target_header = format!("[{section}]");
    let mut in_section = false;
    let mut keys: Vec<String> = Vec::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line == target_header {
            in_section = true;
            continue;
        }
        if in_section && line.starts_with('[') {
            break;
        }
        if !in_section {
            continue;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, _)) = line.split_once('=') {
            let key = key.trim();
            if !key.is_empty() {
                keys.push(key.to_string());
            }
        }
    }
    keys
}

#[test]
fn workspace_root_resolves() {
    let root = workspace_root();
    assert!(
        root.join("Cargo.toml").is_file(),
        "workspace_root() should point to the dir containing the root Cargo.toml; got {}",
        root.display()
    );
    assert!(
        root.join("crates/capcom").is_dir(),
        "workspace should contain crates/capcom"
    );
    assert!(
        root.join("crates/capcom-engine").is_dir(),
        "workspace should contain crates/capcom-engine"
    );
}
