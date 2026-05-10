//! Relationship entity in the labeled property graph data model.
//!
//! Each relationship has:
//! - an internal [`RelationshipId`] assigned by the owning `Graph`,
//! - exactly one [`RelationshipType`] (sibling requirement
//!   `APH-DM-004`),
//! - a directed [`NodeId`] pair: a start node and an end node
//!   (sibling requirement `APH-DM-005`),
//! - a [`PropertyMap`] of scalar values.
//!
//! Whether a downstream query treats the relationship as directed or
//! undirected is a query-time concern (`APH-DM-006`) and lives outside
//! this module. The stored value is always directed.

use crate::data_model::ids::{NodeId, RelationshipId};
use crate::data_model::labels::RelationshipType;
use crate::data_model::properties::PropertyMap;

/// A directed, typed relationship between two nodes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Relationship {
    id: RelationshipId,
    rel_type: RelationshipType,
    start: NodeId,
    end: NodeId,
    properties: PropertyMap,
}

impl Relationship {
    /// Constructs a relationship directly. `pub(crate)` because
    /// allocation flows through `Graph::create_relationship`; this
    /// constructor exists for the graph container and for tests.
    #[must_use]
    pub(crate) fn new(
        id: RelationshipId,
        rel_type: RelationshipType,
        start: NodeId,
        end: NodeId,
        properties: PropertyMap,
    ) -> Self {
        Self {
            id,
            rel_type,
            start,
            end,
            properties,
        }
    }

    #[must_use]
    pub(crate) fn id(&self) -> RelationshipId {
        self.id
    }

    #[must_use]
    pub(crate) fn rel_type(&self) -> &RelationshipType {
        &self.rel_type
    }

    #[must_use]
    pub(crate) fn start(&self) -> NodeId {
        self.start
    }

    #[must_use]
    pub(crate) fn end(&self) -> NodeId {
        self.end
    }

    #[must_use]
    pub(crate) fn properties(&self) -> &PropertyMap {
        &self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::Relationship;
    use crate::data_model::ids::{NodeId, RelationshipId};
    use crate::data_model::labels::RelationshipType;
    use crate::data_model::properties::{PropertyKey, PropertyMap, PropertyValue};

    fn make_relationship_with_property() -> Relationship {
        let mut props = PropertyMap::new();
        props.insert(PropertyKey::new("since"), PropertyValue::Integer(2024));
        Relationship::new(
            RelationshipId::from_raw(1),
            RelationshipType::new("KNOWS"),
            NodeId::from_raw(10),
            NodeId::from_raw(20),
            props,
        )
    }

    #[test]
    fn relationship_exposes_all_constructor_arguments() {
        let rel = make_relationship_with_property();
        assert_eq!(rel.id(), RelationshipId::from_raw(1));
        assert_eq!(rel.rel_type().as_str(), "KNOWS");
        assert_eq!(rel.start(), NodeId::from_raw(10));
        assert_eq!(rel.end(), NodeId::from_raw(20));
    }

    #[test]
    fn relationship_carries_its_property_map() {
        let rel = make_relationship_with_property();
        let key = PropertyKey::new("since");
        assert_eq!(
            rel.properties().get(&key),
            Some(&PropertyValue::Integer(2024))
        );
    }

    #[test]
    fn relationship_direction_is_distinct() {
        let rel = make_relationship_with_property();
        // Direction is preserved at the storage level; query-time
        // direction handling (APH-DM-006) is out of scope here.
        assert_ne!(rel.start(), rel.end());
    }
}
