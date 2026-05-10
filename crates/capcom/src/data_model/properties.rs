//! Property keys, values, and maps.
//!
//! In the labeled property graph required by `APH-DM-001`, both nodes
//! and relationships carry a property map: an ordered set of key /
//! value pairs.
//!
//! `PropertyValue` is the scalar value type the kernel exposes today.
//! It is intentionally minimal — sibling requirement `APH-DM-010` owns
//! the full scalar matrix (BOOLEAN, INTEGER, FLOAT, STRING, DATE, LOCAL
//! TIME, ZONED TIME, LOCAL DATETIME, ZONED DATETIME, DURATION, POINT,
//! VECTOR) and will extend this enum without breaking the call sites
//! that match on it (the `#[non_exhaustive]` attribute below is the
//! contract). Sibling requirements `APH-DM-011`–`016` own list and
//! null semantics; this module deliberately models neither.
//!
//! `PropertyMap` is backed by `BTreeMap` rather than `HashMap`. The
//! kernel needs deterministic iteration order for snapshotting,
//! testing, and any future content-hashing of node/relationship state;
//! `HashMap` would make those flaky and `BTreeMap` is in `std`, so this
//! adds zero dependencies.

use std::collections::BTreeMap;
use std::collections::btree_map::Iter as BTreeMapIter;

/// Property key — an arbitrary non-empty string identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PropertyKey(Box<str>);

impl PropertyKey {
    /// Creates a property key from any string-like value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is empty. The kernel rejects empty keys at the
    /// data-model boundary.
    #[must_use]
    pub(crate) fn new(value: impl Into<Box<str>>) -> Self {
        let boxed: Box<str> = value.into();
        assert!(!boxed.is_empty(), "property key must not be empty");
        Self(boxed)
    }

    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// A scalar property value.
///
/// `#[non_exhaustive]` so future scalar additions under `APH-DM-010`
/// remain non-breaking.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub(crate) enum PropertyValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(Box<str>),
}

impl PropertyValue {
    /// Convenience constructor for the string variant so call sites
    /// don't need to box manually.
    #[must_use]
    pub(crate) fn string(value: impl Into<Box<str>>) -> Self {
        Self::String(value.into())
    }
}

/// Ordered map from property keys to scalar values.
///
/// Backed by `BTreeMap` to keep iteration deterministic. The kernel
/// uses this in tests and will rely on it for any future state hashing
/// or snapshot diffing; `HashMap` is rejected here for non-determinism.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct PropertyMap {
    inner: BTreeMap<PropertyKey, PropertyValue>,
}

impl PropertyMap {
    /// Creates an empty property map.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Inserts a property, returning the previous value if any.
    pub(crate) fn insert(
        &mut self,
        key: PropertyKey,
        value: PropertyValue,
    ) -> Option<PropertyValue> {
        self.inner.insert(key, value)
    }

    /// Looks up a property by key.
    #[must_use]
    pub(crate) fn get(&self, key: &PropertyKey) -> Option<&PropertyValue> {
        self.inner.get(key)
    }

    /// Returns the number of properties stored.
    #[must_use]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the map has no properties.
    #[must_use]
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Iterates over the properties in deterministic key order.
    pub(crate) fn iter(&self) -> BTreeMapIter<'_, PropertyKey, PropertyValue> {
        self.inner.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{PropertyKey, PropertyMap, PropertyValue};

    #[test]
    fn property_key_round_trips() {
        let key = PropertyKey::new("name");
        assert_eq!(key.as_str(), "name");
    }

    #[test]
    #[should_panic(expected = "property key must not be empty")]
    fn empty_property_keys_are_rejected() {
        let _ = PropertyKey::new("");
    }

    #[test]
    fn property_value_variants_compare_by_value() {
        assert_eq!(PropertyValue::Boolean(true), PropertyValue::Boolean(true));
        assert_ne!(PropertyValue::Integer(1), PropertyValue::Integer(2));
        assert_eq!(
            PropertyValue::string("hello"),
            PropertyValue::String(Box::from("hello"))
        );
        assert_eq!(PropertyValue::Float(1.5), PropertyValue::Float(1.5));
    }

    #[test]
    fn property_map_starts_empty() {
        let map = PropertyMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn property_map_inserts_and_looks_up_values() {
        let mut map = PropertyMap::new();
        let key = PropertyKey::new("age");
        assert!(
            map.insert(key.clone(), PropertyValue::Integer(30))
                .is_none()
        );
        assert_eq!(map.get(&key), Some(&PropertyValue::Integer(30)));
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn property_map_overwrite_returns_previous_value() {
        let mut map = PropertyMap::new();
        let key = PropertyKey::new("status");
        map.insert(key.clone(), PropertyValue::string("draft"));
        let prev = map.insert(key.clone(), PropertyValue::string("active"));
        assert_eq!(prev, Some(PropertyValue::string("draft")));
        assert_eq!(map.get(&key), Some(&PropertyValue::string("active")));
    }

    #[test]
    fn property_map_iterates_in_deterministic_key_order() {
        let mut map = PropertyMap::new();
        // Insert out of order; iteration must come back sorted by key.
        map.insert(PropertyKey::new("zeta"), PropertyValue::Integer(1));
        map.insert(PropertyKey::new("alpha"), PropertyValue::Integer(2));
        map.insert(PropertyKey::new("mu"), PropertyValue::Integer(3));

        let keys: Vec<&str> = map.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["alpha", "mu", "zeta"]);
    }
}
