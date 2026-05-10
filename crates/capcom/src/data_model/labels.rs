//! Label and relationship-type vocabulary.
//!
//! In the labeled property graph data model required by `APH-DM-001`,
//! every node may carry zero or more *labels* and every relationship
//! carries exactly one *type*. Both are user-supplied strings with no
//! global registry — sibling requirement `APH-DM-026` (open-schema
//! operation) makes that explicit, and the preflight forbids forcing a
//! schema-first catalog here.
//!
//! `Label` and `RelationshipType` are typed wrappers over `Box<str>`
//! rather than bare strings. This keeps the API self-documenting and
//! gives the kernel a single place to add interning, validation, or
//! cross-graph deduplication later without rewriting call sites.

use core::fmt;

/// A label that may be attached to a node. Labels are arbitrary
/// non-empty strings; the kernel imposes no further structure here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Label(Box<str>);

impl Label {
    /// Creates a label from any string-like value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is empty. The kernel rejects empty labels at
    /// the data-model boundary so storage and query layers further down
    /// can assume non-empty content.
    #[must_use]
    pub(crate) fn new(value: impl Into<Box<str>>) -> Self {
        let boxed: Box<str> = value.into();
        assert!(!boxed.is_empty(), "label must not be empty");
        Self(boxed)
    }

    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The type of a directed relationship. Each relationship has exactly
/// one type (sibling requirement `APH-DM-004` will pin that constraint
/// at the relationship level).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RelationshipType(Box<str>);

impl RelationshipType {
    /// Creates a relationship type from any string-like value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is empty.
    #[must_use]
    pub(crate) fn new(value: impl Into<Box<str>>) -> Self {
        let boxed: Box<str> = value.into();
        assert!(!boxed.is_empty(), "relationship type must not be empty");
        Self(boxed)
    }

    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Label, RelationshipType};

    #[test]
    fn labels_round_trip_their_inner_string() {
        let label = Label::new("Person");
        assert_eq!(label.as_str(), "Person");
        assert_eq!(label.to_string(), "Person");
    }

    #[test]
    fn relationship_types_round_trip_their_inner_string() {
        let rel_type = RelationshipType::new("KNOWS");
        assert_eq!(rel_type.as_str(), "KNOWS");
        assert_eq!(rel_type.to_string(), "KNOWS");
    }

    #[test]
    #[should_panic(expected = "label must not be empty")]
    fn empty_labels_are_rejected() {
        let _ = Label::new("");
    }

    #[test]
    #[should_panic(expected = "relationship type must not be empty")]
    fn empty_relationship_types_are_rejected() {
        let _ = RelationshipType::new("");
    }

    #[test]
    fn labels_can_be_compared_and_sorted() {
        let mut labels = [
            Label::new("Person"),
            Label::new("Animal"),
            Label::new("Place"),
        ];
        labels.sort();
        assert_eq!(labels[0].as_str(), "Animal");
        assert_eq!(labels[1].as_str(), "Person");
        assert_eq!(labels[2].as_str(), "Place");
    }
}
