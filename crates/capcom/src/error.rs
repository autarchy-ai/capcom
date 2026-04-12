//! Kernel-wide error type.
//!
//! Per the `APH-DM-001` architecture preflight, the capcom kernel
//! introduces error handling once and reuses it across model, storage,
//! and host concerns rather than spawning parallel hierarchies. This
//! module owns that single type. New variants land here as new
//! requirements are implemented.
//!
//! `KernelError` is `pub(crate)` for now: the kernel's public surface is
//! still just `version()`. When the kernel exposes operations that can
//! fail, the type (or a curated re-export of it) will graduate to `pub`
//! in the same change that adds those operations and the matching entry
//! in `crates/capcom-arch-tests/tests/kernel_boundary.rs`.

use core::fmt;

use crate::data_model::ids::NodeId;

/// Errors raised by capcom kernel operations.
///
/// Marked `#[non_exhaustive]` so future requirement work can add variants
/// without breaking match sites inside the crate (and outside, when this
/// type is later promoted to the public surface).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum KernelError {
    /// A graph operation referenced a node id that does not exist in
    /// the graph it was applied to. Raised by relationship creation when
    /// `start` or `end` is unknown.
    UnknownNode(NodeId),
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode(id) => write!(f, "unknown node id: {id}"),
        }
    }
}

impl std::error::Error for KernelError {}

#[cfg(test)]
mod tests {
    use super::{KernelError, NodeId};

    #[test]
    fn unknown_node_displays_with_id() {
        let err = KernelError::UnknownNode(NodeId::from_raw(7));
        assert_eq!(err.to_string(), "unknown node id: 7");
    }

    #[test]
    fn kernel_error_is_std_error() {
        let err: Box<dyn std::error::Error> =
            Box::new(KernelError::UnknownNode(NodeId::from_raw(1)));
        assert!(err.to_string().contains("unknown node id"));
    }
}
