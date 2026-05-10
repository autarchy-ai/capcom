//! Capcom: the graph database engine kernel.
//!
//! This crate is the product-owned kernel for
//! [Aphelion](https://github.com/KeplerOps/aphelion), per ADR-001.
//! V1 is single-node only (ADR-003, ADR-005, ADR-009).
//!
//! The public API surface is intentionally minimal. The `data_model`
//! module that satisfies `APH-DM-001` (labeled property graph) is
//! `pub(crate)` per its architecture preflight; growing the public
//! surface requires a paired update to
//! `crates/capcom-arch-tests/tests/kernel_boundary.rs` in the same
//! change.

// `data_model` and `error` are crate-internal until later requirements
// expose graph operations through the kernel's public API. Until then,
// most items are exercised only by intra-crate tests, which is exactly
// what the preflight at `docs/APH-DM-001-preflight.md` calls for. The
// dead-code allow prevents clippy `-D warnings` from blocking that
// intentional state; it lifts as soon as a sibling requirement (storage,
// txn, query) starts wiring graph types into runtime call paths.
#[allow(dead_code)]
pub(crate) mod data_model;
#[allow(dead_code)]
pub(crate) mod error;

/// Returns the capcom kernel version from `CARGO_PKG_VERSION`.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }
}
