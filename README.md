# capcom

The product-owned graph database engine kernel for
[Aphelion](https://github.com/KeplerOps/aphelion), implemented in Rust.
See [ADR-001](https://github.com/KeplerOps/aphelion/blob/main/docs/adrs/ADR-001.md)
for background and
[ADR-012](https://github.com/KeplerOps/aphelion/blob/main/docs/adrs/ADR-012.md)
for the co-located out-of-process integration model.

## Status

Pre-alpha. This repo currently contains project scaffolding only. The
kernel itself (storage, data model, transactions, query execution)
lands in subsequent work order steps -- see
[`aphelion/notes/work-order.md`](https://github.com/KeplerOps/aphelion/blob/main/notes/work-order.md)
for the full build order.

## Build

Requires Rust stable 1.85+ with `rustfmt` and `clippy` components. The
`rust-toolchain.toml` pin is honored automatically by rustup.

| Task | Command |
|---|---|
| Build | `cargo build --workspace` |
| Test | `cargo test --workspace` |
| Format (check) | `cargo fmt --all --check` |
| Format (fix) | `cargo fmt --all` |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` |
| ADR conformance | `scripts/check-adr-conformance.sh` |

## Layout

```
.
├── Cargo.toml                     # workspace manifest
├── rust-toolchain.toml            # pinned toolchain
├── rustfmt.toml                   # formatter config
├── crates/
│   ├── capcom/                    # library crate -- the kernel itself
│   │   ├── Cargo.toml
│   │   └── src/lib.rs             # currently a stub (version placeholder)
│   └── capcom-engine/             # binary crate -- the engine node process
│       ├── Cargo.toml
│       └── src/main.rs            # currently a stub
├── scripts/
│   └── check-adr-conformance.sh   # grep-based ADR enforcement
├── .github/workflows/ci.yml       # adr-conformance / lint / test jobs
├── .ground-control.yaml           # Ground Control workflow config for this repo
└── .claude/hooks/adr-boundary-check.sh   # PreToolUse hook for Claude Code
```

## Architectural constraints

The scripts and hooks below enforce Aphelion's ADR decisions against
the kernel at every edit and every CI run.

- **ADR-001**: No references to MillenniumDB or Graphflow. Capcom is the
  chosen engine; competing engines are rejected.
- **ADR-003 / ADR-005 / ADR-009**: No distributed systems primitives --
  consensus, replication, failover, cross-node operations, distributed
  transactions, shared storage. V1 is single-node only.
- **ADR-008**: Public wire protocol choice is deferred -- there is no
  public protocol commitment yet.
- **ADR-012**: Capcom runs as a co-located out-of-process engine node
  behind a product-owned IPC contract. The binary crate
  (`crates/capcom-engine`) is the eventual entry point for that process.

Overrides exist but require a `// adr-override: ADR-NNN -- <rationale>`
comment next to the offending code.
