//! Labeled property graph data model (`APH-DM-001`).
//!
//! This module is the kernel's authoritative implementation of the
//! labeled property graph (LPG) data model. Per the architecture
//! preflight at `docs/APH-DM-001-preflight.md`, every type defined here
//! is `pub(crate)`: the public surface of the `capcom` crate stays
//! limited to `version()` until a reviewed kernel API genuinely needs
//! to expose graph types, at which point this module's re-exports and
//! `crates/capcom-arch-tests/tests/kernel_boundary.rs` get a paired
//! update in the same change.
//!
//! ## Module layout
//!
//! - [`ids`] — internal entity identifiers
//! - [`labels`] — node labels and relationship types
//! - [`properties`] — property keys, values, and ordered maps
//! - [`node`] — `Node` value type
//! - [`relationship`] — `Relationship` value type
//! - [`graph`] — `Graph` in-memory container with id allocation
//!
//! Sibling requirements `APH-DM-002`–`APH-DM-026`, plus all of
//! `APH-STO-*` and `APH-TXN-*`, build on top of this module. Each will
//! land in its own work order; this module deliberately does not
//! preempt them.

pub(crate) mod graph;
pub(crate) mod ids;
pub(crate) mod labels;
pub(crate) mod node;
pub(crate) mod properties;
pub(crate) mod relationship;
