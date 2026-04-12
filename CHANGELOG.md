# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-04-11

### Added

- `crates/capcom/src/data_model/` module satisfying `APH-DM-001`
  (labeled property graph data model). Submodules: `ids`, `labels`,
  `properties`, `node`, `relationship`, `graph`. All types are
  `pub(crate)` per the architecture preflight; the kernel's public
  surface remains `capcom::version()`.
- `crates/capcom/src/error.rs` introducing `KernelError`, the single
  kernel-wide error enum (one variant today: `UnknownNode`). Future
  kernel error variants land here per preflight guidance.
- `docs/README.md` and `docs/design/data-model.md`. The design note
  documents the type vocabulary and embeds Mermaid class and sequence
  diagrams covering `Graph`, `Node`, `Relationship`, `PropertyMap`,
  `PropertyValue`, identifiers, and the `KernelError` path.
- `docs/APH-DM-001-preflight.md` recording the codex architecture
  preflight guardrails for `APH-DM-001`.
- 33 in-crate tests covering id round-trips, label / relationship-type
  construction, property map insertion and deterministic iteration
  order, node and relationship value semantics, and the full LPG
  semantics of `Graph::create_node` / `create_relationship` / lookup
  including the `KernelError::UnknownNode` error path.

### Changed

- `crates/capcom/src/lib.rs` declares `data_model` and `error` as
  `pub(crate)` modules behind `#[allow(dead_code)]` (rationale inline)
  until sibling storage / txn / query requirements wire them into
  runtime call paths.
- `README.md` adds a "Kernel surface" / "Data model" section, refreshes
  the layout block to include `docs/` and the `data_model/` source
  tree, and links the new design note and preflight.
- `AGENTS.md` Repository Structure block adds `crates/`, `docs/`,
  `docs/design/`, and `docs/adrs/` (with a note that aphelion ADRs are
  authoritative and no local ADRs exist yet).

## [0.2.0] - 2026-04-11

### Added

- Cargo workspace structure with three crates:
  - `crates/capcom/` -- library crate, placeholder kernel API
  - `crates/capcom-engine/` -- binary crate, placeholder for the future
    co-located out-of-process engine node (ADR-012)
  - `crates/capcom-arch-tests/` -- architecture-test crate enforcing
    workspace-level invariants via `cargo test`. Three test suites:
    `no_distributed_code` (ADR-003/005/009), `no_banned_engines`
    (ADR-001), and `kernel_boundary` (public API surface + engine
    dependency closure)
- `rust-toolchain.toml` pinning the stable channel with `rustfmt` and
  `clippy` components (minimal profile)
- `rustfmt.toml` setting edition 2024
- `scripts/check-adr-conformance.sh` enforcing ADR-001 (no banned
  engines: MillenniumDB, Graphflow) and ADR-003/005/009 (no distributed
  primitives) against Rust source and `Cargo.toml` manifests
- `.claude/hooks/adr-boundary-check.sh` applying the same checks as a
  PreToolUse hook on Edit/Write operations
- CI pipeline with three real jobs (replacing the TODO placeholders):
  `adr-conformance`, `lint` (cargo fmt + clippy), `test` (cargo build +
  cargo test)
- Pre-commit hooks: `cargo fmt --all --check` (Stage 3) and
  `scripts/check-adr-conformance.sh` (Stage 4)
- README with project summary, build/test/run commands, layout, and
  the enforced architectural constraints

### Changed

- `.ground-control.yaml` workflow block populated with cargo commands
  (previously commented out pending scaffolding)
- `.gitignore` adds Rust entries (`target/`, `*.pdb`)
- `.editorconfig` adds `[*.rs]` and `[*.toml]` sections
- `.claude/settings.json` registers `adr-boundary-check.sh` alongside
  the existing `protect_files.sh` PreToolUse hook
- `README.md` replaces the "Project Name" placeholder with real content

## [0.1.0] - 2026-04-11

### Added

- `.ground-control.yaml` declaring capcom's Ground Control project id
  and reserving commented slots for workflow commands (to be filled in
  with the first Cargo scaffolding work order step).

### Changed

- `AGENTS.md` Ground Control Context section now points to
  `.ground-control.yaml`; added a brief note tying capcom to Aphelion
  ADR-001 and the V1 single-node constraint.
- `CLAUDE.md` Project / Build / Ground Control TODO placeholders
  replaced with concrete content.
- `.mcp.json` `GH_REPO` placeholder replaced with `KeplerOps/capcom`.
