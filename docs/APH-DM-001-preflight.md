# APH-DM-001 Architecture Preflight

- Requirement: `APH-DM-001`
- Issue: `#3`
- Status: pre-implementation guidance

## Ownership and Boundary

`APH-DM-001` is kernel-owned per Aphelion `ADR-016`. The authoritative
labeled property graph model belongs in `crates/capcom`, not in
`crates/capcom-engine`, which remains the thin co-located host process
per `ADR-012` and `ADR-014`.

Do not let future IPC DTOs, query AST types, storage-page structs, or
engine-host helpers become the canonical graph model. The kernel model
must be the single source of truth for graph semantics.

## Guardrails

- Keep the authoritative graph model in `capcom`; keep
  `capcom-engine` focused on process/bootstrap concerns and depending
  only on `capcom`, matching
  `crates/capcom-arch-tests/tests/kernel_boundary.rs`.
- Preserve the minimal public API discipline already enforced by
  `kernel_boundary.rs`. New graph-model types should stay private or
  `pub(crate)` until a reviewed kernel API truly needs them. Any `pub`
  surface growth must land with an explicit architecture-test update.
- Reuse the existing ADR enforcement already wired into
  `.claude/hooks/adr-boundary-check.sh`,
  `scripts/check-adr-conformance.sh`, and
  `crates/capcom-arch-tests/tests/no_distributed_code.rs`. No
  clustering, replication, cross-node identifiers, or HA leakage.
- Treat query-visible graph semantics and storage-visible graph
  encoding as compatibility-surface changes under Aphelion `ADR-010`,
  not as private refactors.
- Keep dependency growth deliberate. New crates require the same
  architectural scrutiny already called out in `kernel_boundary.rs`;
  do not add runtime dependencies to `capcom-engine`, and do not add
  parser, protocol, or serialization packages unless the graph model
  itself genuinely needs them.

## Required Reuse of Existing Cross-Cutting Concerns

- Reuse the current crate split as the first architectural boundary:
  `crates/capcom` owns graph semantics, `crates/capcom-engine` owns
  hosting concerns, and `crates/capcom-arch-tests` owns boundary
  enforcement.
- Keep validation single-owner. Graph-structure and value invariants
  belong in the kernel model. Process/bootstrap validation belongs in
  `capcom-engine`. Future query, IPC, or API validation must stay at
  those boundaries instead of being copied into core model types.
- Introduce kernel error handling once and reuse it consistently. Do
  not create separate invariant-failure hierarchies in model, storage,
  engine host, and future adapters that all describe the same class of
  graph-model errors differently.
- Keep observability at operation boundaries, not inside pure value
  types. If logging or metrics are added, keep graph entities/value
  objects free of ad hoc logging behavior, and keep correlation/lifecycle
  context at engine, session, or transaction boundaries rather than
  threading it through model structs.
- Keep persistence concerns separate from the logical graph model.
  Storage addresses, WAL positions, free-list state, page-layout tags,
  and recovery metadata are not graph nodes, relationships, labels, or
  properties.
- Reuse the existing workflow and enforcement path:
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo build --workspace --all-targets`, `cargo test --workspace --all-targets`,
  and `scripts/check-adr-conformance.sh`. Extend architecture tests when
  public-surface or crate-boundary expectations change.

## Gotchas and Anti-Patterns to Avoid

- Do not conflate the labeled property graph model with the query
  parser or planner AST. Query trees may reference graph semantics, but
  they are not the engine's canonical data model.
- Do not conflate the graph model with future IPC or wire DTOs. The
  product-owned boundary from `ADR-014` is capability-oriented and
  handle-based; it must not become a disguised mirror of kernel structs.
- Do not conflate the graph model with storage layout. Internal page or
  slot identifiers are not durable external graph identity.
- Do not introduce a mandatory global schema registry or schema-first
  type catalog as part of this requirement. `APH-DM-026` makes
  open-schema operation explicit, so `APH-DM-001` must not force the
  opposite model up front.
- Do not over-specify scalar, null, temporal, spatial, vector, path, or
  identity semantics that are covered by later `APH-DM-*` requirements.
  `APH-DM-001` should establish the graph-model boundary, not preempt
  the entire value-system backlog.
- Do not create duplicate node, relationship, label, or property
  representations across `capcom`, `capcom-engine`, and future boundary
  adapters.
- Do not use a mega-enum or god-struct that mixes graph entities, query
  syntax tags, storage records, IPC handles, and metadata ownership into
  one abstraction.

## Non-Goals and Implementation Boundaries

- No query-language, planner, or execution-engine design as part of this
  requirement.
- No public wire protocol, driver contract, or product-facing DTO shape.
- No storage durability, WAL, recovery, checkpoint, or space-reuse
  semantics beyond keeping the model cleanly separable from those future
  concerns.
- No DBMS-level catalog, security-policy, extension-policy, or topology
  metadata mixed into the graph data model.
- No distributed, clustered, replicated, or HA-oriented semantics in V1.

Existing ADRs already provide the needed architecture decisions here.
No new ADR is warranted unless implementation uncovers a real conflict
between graph-model authority, compatibility-surface discipline, and the
product/kernel boundary.
