# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-04-11

### Added

- Cargo workspace structure with two crates:
  - `crates/capcom/` -- library crate, placeholder kernel API
  - `crates/capcom-engine/` -- binary crate, placeholder for the future
    co-located out-of-process engine node (ADR-012)
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
