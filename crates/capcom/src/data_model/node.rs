//! Node entity in the labeled property graph data model.
//!
//! Each node carries:
//! - an internal [`NodeId`] assigned by the owning `Graph`,
//! - a possibly-empty set of labels (sibling requirement
//!   `APH-DM-003` formalises "zero or more labels per node"),
//! - a [`PropertyMap`] of scalar values.
//!
//! `Node` is a plain owned value with no implicit coupling to a graph
//! container — the preflight is explicit that pure value types must
//! stay free of cross-cutting concerns like logging, lifecycle, or IO.
//! `Graph` allocates ids and stores nodes; everything else lives here.

use std::collections::BTreeSet;
use std::collections::btree_set::Iter as BTreeSetIter;

use crate::data_model::ids::NodeId;
use crate::data_model::labels::Label;
use crate::data_model::properties::PropertyMap;

/// A node in the labeled property graph.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Node {
    id: NodeId,
    labels: BTreeSet<Label>,
    properties: PropertyMap,
}

impl Node {
    /// Constructs a node directly. `pub(crate)` because allocation
    /// flows through `Graph::create_node`; this constructor exists for
    /// the graph container and for tests.
    #[must_use]
    pub(crate) fn new(id: NodeId, labels: BTreeSet<Label>, properties: PropertyMap) -> Self {
        Self {
            id,
            labels,
            properties,
        }
    }

    /// Returns the node's internal id.
    #[must_use]
    pub(crate) fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the number of labels attached to this node.
    #[must_use]
    pub(crate) fn label_count(&self) -> usize {
        self.labels.len()
    }

    /// Returns `true` if the node carries the given label.
    #[must_use]
    pub(crate) fn has_label(&self, label: &Label) -> bool {
        self.labels.contains(label)
    }

    /// Iterates over the labels in deterministic (sorted) order.
    pub(crate) fn labels(&self) -> BTreeSetIter<'_, Label> {
        self.labels.iter()
    }

    /// Borrows the property map.
    #[must_use]
    pub(crate) fn properties(&self) -> &PropertyMap {
        &self.properties
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use crate::data_model::ids::NodeId;
    use crate::data_model::labels::Label;
    use crate::data_model::properties::{PropertyKey, PropertyMap, PropertyValue};
    use std::collections::BTreeSet;

    fn id() -> NodeId {
        NodeId::from_raw(1)
    }

    #[test]
    fn node_with_no_labels_reports_zero() {
        let node = Node::new(id(), BTreeSet::new(), PropertyMap::new());
        assert_eq!(node.label_count(), 0);
        assert!(node.labels().next().is_none());
    }

    #[test]
    fn node_with_multiple_labels_reports_them_in_order() {
        let mut labels = BTreeSet::new();
        labels.insert(Label::new("Person"));
        labels.insert(Label::new("Employee"));
        labels.insert(Label::new("Admin"));
        let node = Node::new(id(), labels, PropertyMap::new());

        assert_eq!(node.label_count(), 3);
        assert!(node.has_label(&Label::new("Person")));
        assert!(node.has_label(&Label::new("Admin")));
        assert!(node.has_label(&Label::new("Employee")));
        assert!(!node.has_label(&Label::new("Customer")));

        let listed: Vec<&str> = node.labels().map(Label::as_str).collect();
        assert_eq!(listed, vec!["Admin", "Employee", "Person"]);
    }

    #[test]
    fn node_carries_its_property_map() {
        let mut props = PropertyMap::new();
        props.insert(PropertyKey::new("name"), PropertyValue::string("Ada"));
        let node = Node::new(id(), BTreeSet::new(), props);

        let key = PropertyKey::new("name");
        assert_eq!(
            node.properties().get(&key),
            Some(&PropertyValue::string("Ada"))
        );
    }

    #[test]
    fn node_returns_its_id() {
        let node = Node::new(NodeId::from_raw(99), BTreeSet::new(), PropertyMap::new());
        assert_eq!(node.id(), NodeId::from_raw(99));
    }
}
