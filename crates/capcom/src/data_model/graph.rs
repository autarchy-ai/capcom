//! In-memory `Graph` container.
//!
//! `Graph` is the kernel's day-zero in-memory implementation of the
//! labeled property graph required by `APH-DM-001`. Storage durability
//! (`APH-STO-*`), transactions (`APH-TXN-*`), and any persistence
//! concerns are deliberately *not* part of this type — the preflight
//! is explicit that page layout, free lists, WAL state, and recovery
//! metadata must not collapse into the data model.
//!
//! Identifiers are monotonic `u64`s allocated inside the graph instance.
//! Two `Graph` instances do not share an id space; cross-graph identity
//! is the responsibility of `APH-DM-025` (external element identity)
//! and `APH-STO-014` (stable external identity), which are out of scope
//! here.

use std::collections::BTreeMap;
use std::collections::btree_map::Values as BTreeMapValues;

use crate::data_model::ids::{NodeId, RelationshipId};
use crate::data_model::labels::{Label, RelationshipType};
use crate::data_model::node::Node;
use crate::data_model::properties::PropertyMap;
use crate::data_model::relationship::Relationship;
use crate::error::KernelError;

/// In-memory labeled property graph container.
#[derive(Debug, Clone, Default)]
pub(crate) struct Graph {
    nodes: BTreeMap<NodeId, Node>,
    relationships: BTreeMap<RelationshipId, Relationship>,
    next_node_id: u64,
    next_relationship_id: u64,
}

impl Graph {
    /// Creates a new, empty graph.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Creates a node with the given labels and properties, returning
    /// its newly-allocated id.
    pub(crate) fn create_node(
        &mut self,
        labels: impl IntoIterator<Item = Label>,
        properties: PropertyMap,
    ) -> NodeId {
        let id = NodeId::from_raw(self.next_node_id);
        self.next_node_id = self
            .next_node_id
            .checked_add(1)
            .expect("node id space exhausted");

        let label_set = labels.into_iter().collect();
        let node = Node::new(id, label_set, properties);
        self.nodes.insert(id, node);
        id
    }

    /// Creates a directed, typed relationship between two existing
    /// nodes.
    ///
    /// # Errors
    ///
    /// Returns `KernelError::UnknownNode` if either `start` or `end`
    /// does not exist in this graph.
    pub(crate) fn create_relationship(
        &mut self,
        start: NodeId,
        end: NodeId,
        rel_type: RelationshipType,
        properties: PropertyMap,
    ) -> Result<RelationshipId, KernelError> {
        if !self.nodes.contains_key(&start) {
            return Err(KernelError::UnknownNode(start));
        }
        if !self.nodes.contains_key(&end) {
            return Err(KernelError::UnknownNode(end));
        }

        let id = RelationshipId::from_raw(self.next_relationship_id);
        self.next_relationship_id = self
            .next_relationship_id
            .checked_add(1)
            .expect("relationship id space exhausted");

        let relationship = Relationship::new(id, rel_type, start, end, properties);
        self.relationships.insert(id, relationship);
        Ok(id)
    }

    /// Looks up a node by id.
    #[must_use]
    pub(crate) fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Looks up a relationship by id.
    #[must_use]
    pub(crate) fn relationship(&self, id: RelationshipId) -> Option<&Relationship> {
        self.relationships.get(&id)
    }

    /// Number of nodes currently in the graph.
    #[must_use]
    pub(crate) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of relationships currently in the graph.
    #[must_use]
    pub(crate) fn relationship_count(&self) -> usize {
        self.relationships.len()
    }

    /// Iterates over all nodes in id order.
    pub(crate) fn iter_nodes(&self) -> BTreeMapValues<'_, NodeId, Node> {
        self.nodes.values()
    }

    /// Iterates over all relationships in id order.
    pub(crate) fn iter_relationships(&self) -> BTreeMapValues<'_, RelationshipId, Relationship> {
        self.relationships.values()
    }
}

#[cfg(test)]
mod tests {
    //! Full LPG-semantics tests for `APH-DM-001`.
    //!
    //! These tests live inside the crate so the data-model types can
    //! stay `pub(crate)` per the preflight's minimal-public-API
    //! discipline. Each test maps to a clause of `APH-DM-001`'s
    //! statement: "The system shall implement a labeled property graph
    //! data model."

    use super::Graph;
    use crate::data_model::labels::{Label, RelationshipType};
    use crate::data_model::properties::{PropertyKey, PropertyMap, PropertyValue};
    use crate::error::KernelError;

    fn property_map_with(key: &str, value: PropertyValue) -> PropertyMap {
        let mut map = PropertyMap::new();
        map.insert(PropertyKey::new(key), value);
        map
    }

    #[test]
    fn nodes_can_carry_zero_or_more_labels() {
        let mut graph = Graph::new();

        let unlabeled = graph.create_node([], PropertyMap::new());
        let one_label = graph.create_node([Label::new("Person")], PropertyMap::new());
        let many_labels = graph.create_node(
            [
                Label::new("Person"),
                Label::new("Employee"),
                Label::new("Admin"),
            ],
            PropertyMap::new(),
        );

        assert_eq!(graph.node(unlabeled).unwrap().label_count(), 0);
        assert_eq!(graph.node(one_label).unwrap().label_count(), 1);
        assert!(
            graph
                .node(one_label)
                .unwrap()
                .has_label(&Label::new("Person"))
        );
        let many = graph.node(many_labels).unwrap();
        assert_eq!(many.label_count(), 3);
        assert!(many.has_label(&Label::new("Person")));
        assert!(many.has_label(&Label::new("Employee")));
        assert!(many.has_label(&Label::new("Admin")));
    }

    #[test]
    fn relationships_are_typed_directed_and_link_existing_nodes() {
        let mut graph = Graph::new();
        let alice = graph.create_node([Label::new("Person")], PropertyMap::new());
        let bob = graph.create_node([Label::new("Person")], PropertyMap::new());

        let knows = graph
            .create_relationship(
                alice,
                bob,
                RelationshipType::new("KNOWS"),
                PropertyMap::new(),
            )
            .expect("both endpoints exist");

        let stored = graph.relationship(knows).expect("relationship round-trips");
        assert_eq!(stored.rel_type().as_str(), "KNOWS");
        assert_eq!(stored.start(), alice);
        assert_eq!(stored.end(), bob);
        assert_ne!(stored.start(), stored.end());
    }

    #[test]
    fn nodes_and_relationships_both_carry_property_maps() {
        let mut graph = Graph::new();
        let alice = graph.create_node(
            [Label::new("Person")],
            property_map_with("name", PropertyValue::string("Alice")),
        );
        let bob = graph.create_node(
            [Label::new("Person")],
            property_map_with("name", PropertyValue::string("Bob")),
        );
        let knows = graph
            .create_relationship(
                alice,
                bob,
                RelationshipType::new("KNOWS"),
                property_map_with("since", PropertyValue::Integer(2026)),
            )
            .unwrap();

        let alice_props = graph.node(alice).unwrap().properties();
        let key_name = PropertyKey::new("name");
        assert_eq!(
            alice_props.get(&key_name),
            Some(&PropertyValue::string("Alice"))
        );

        let knows_props = graph.relationship(knows).unwrap().properties();
        let key_since = PropertyKey::new("since");
        assert_eq!(
            knows_props.get(&key_since),
            Some(&PropertyValue::Integer(2026))
        );
    }

    #[test]
    fn graph_can_hold_many_entities_and_look_them_up_by_id() {
        let mut graph = Graph::new();
        let mut node_ids = Vec::new();
        for i in 0..5 {
            let id = graph.create_node(
                [Label::new("Item")],
                property_map_with("rank", PropertyValue::Integer(i)),
            );
            node_ids.push(id);
        }

        let mut rel_ids = Vec::new();
        for window in node_ids.windows(2) {
            let rel = graph
                .create_relationship(
                    window[0],
                    window[1],
                    RelationshipType::new("NEXT"),
                    PropertyMap::new(),
                )
                .unwrap();
            rel_ids.push(rel);
        }

        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.relationship_count(), 4);
        assert_eq!(graph.iter_nodes().count(), 5);
        assert_eq!(graph.iter_relationships().count(), 4);

        for id in node_ids {
            assert!(graph.node(id).is_some());
        }
        for id in rel_ids {
            assert!(graph.relationship(id).is_some());
        }
    }

    #[test]
    fn create_relationship_rejects_unknown_start_node() {
        let mut graph = Graph::new();
        let bob = graph.create_node([], PropertyMap::new());

        let phantom = crate::data_model::ids::NodeId::from_raw(999);
        let err = graph
            .create_relationship(
                phantom,
                bob,
                RelationshipType::new("KNOWS"),
                PropertyMap::new(),
            )
            .unwrap_err();
        assert_eq!(err, KernelError::UnknownNode(phantom));
    }

    #[test]
    fn create_relationship_rejects_unknown_end_node() {
        let mut graph = Graph::new();
        let alice = graph.create_node([], PropertyMap::new());

        let phantom = crate::data_model::ids::NodeId::from_raw(999);
        let err = graph
            .create_relationship(
                alice,
                phantom,
                RelationshipType::new("KNOWS"),
                PropertyMap::new(),
            )
            .unwrap_err();
        assert_eq!(err, KernelError::UnknownNode(phantom));
    }

    #[test]
    fn ids_are_monotonic_within_a_graph() {
        let mut graph = Graph::new();
        let n0 = graph.create_node([], PropertyMap::new());
        let n1 = graph.create_node([], PropertyMap::new());
        let n2 = graph.create_node([], PropertyMap::new());
        assert!(n0 < n1);
        assert!(n1 < n2);
    }
}
