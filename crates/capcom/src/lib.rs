//! Capcom: the graph database engine kernel.
//!
//! This crate is the product-owned kernel for
//! [Aphelion](https://github.com/KeplerOps/aphelion), per ADR-001.
//! V1 is single-node only (ADR-003, ADR-005, ADR-009).
//!
//! The public API surface is intentionally minimal until work order
//! step 2 (storage, data model, transactions) lands.

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
