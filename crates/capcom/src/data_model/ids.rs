//! Internal entity identifiers for the kernel data model.
//!
//! `NodeId` and `RelationshipId` are newtypes around `u64`. The newtype
//! wrapper isolates downstream call sites from the underlying integer
//! representation, so future work on external identity (`APH-DM-025`)
//! and stable cross-restart identity (`APH-STO-014`) can swap the
//! internals without touching call sites.
//!
//! These identifiers are *internal*: they are valid only within a single
//! `Graph` instance and carry no durability or addressing semantics. The
//! preflight (`docs/APH-DM-001-preflight.md`) is explicit that storage
//! addresses, page slot tags, and external durable identity must not
//! collapse into this type.

use core::fmt;

/// Internal identifier for a node within a single `Graph`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NodeId(u64);

impl NodeId {
    /// Wraps a raw `u64` into a `NodeId`. `pub(crate)` because raw id
    /// construction is reserved for `Graph`'s allocator and for tests.
    #[must_use]
    pub(crate) const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw integer for diagnostics, formatting, or
    /// hashing into deterministic structures inside the kernel.
    #[must_use]
    pub(crate) const fn as_raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Internal identifier for a relationship within a single `Graph`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RelationshipId(u64);

impl RelationshipId {
    #[must_use]
    pub(crate) const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub(crate) const fn as_raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for RelationshipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{NodeId, RelationshipId};

    #[test]
    fn node_id_round_trips_through_raw() {
        let id = NodeId::from_raw(42);
        assert_eq!(id.as_raw(), 42);
    }

    #[test]
    fn relationship_id_round_trips_through_raw() {
        let id = RelationshipId::from_raw(7);
        assert_eq!(id.as_raw(), 7);
    }

    #[test]
    fn node_ids_are_orderable_by_raw() {
        let mut ids = [
            NodeId::from_raw(3),
            NodeId::from_raw(1),
            NodeId::from_raw(2),
        ];
        ids.sort();
        assert_eq!(
            ids,
            [
                NodeId::from_raw(1),
                NodeId::from_raw(2),
                NodeId::from_raw(3),
            ]
        );
    }

    #[test]
    fn ids_format_as_their_raw_value() {
        assert_eq!(NodeId::from_raw(11).to_string(), "11");
        assert_eq!(RelationshipId::from_raw(13).to_string(), "13");
    }
}
