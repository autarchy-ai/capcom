# capcom

The product-owned graph database engine kernel for
[Aphelion](https://github.com/KeplerOps/aphelion), implemented in Rust.
See [ADR-001](https://github.com/KeplerOps/aphelion/blob/main/docs/adrs/ADR-001.md)
for background and
[ADR-012](https://github.com/KeplerOps/aphelion/blob/main/docs/adrs/ADR-012.md)
for the co-located out-of-process integration model.

## Status

Pre-alpha. The kernel currently has:

- Project scaffolding (workspace, CI, ADR-conformance enforcement,
  architecture tests).
- The `data_model` module satisfying `APH-DM-001` (labeled property
  graph) — kernel-internal, not yet on the public API. See
  [`docs/design/data-model.md`](docs/design/data-model.md) and the
  [preflight note](docs/APH-DM-001-preflight.md).

Storage, transactions, query execution, and the rest of the kernel
land in subsequent work order steps -- see
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
│   │   └── src/
│   │       ├── lib.rs             # public API surface (currently just version())
│   │       ├── error.rs           # KernelError (crate-internal)
│   │       └── data_model/        # APH-DM-001 labeled property graph
│   │           ├── mod.rs
│   │           ├── ids.rs
│   │           ├── labels.rs
│   │           ├── properties.rs
│   │           ├── node.rs
│   │           ├── relationship.rs
│   │           └── graph.rs
│   ├── capcom-engine/             # binary crate -- the engine node process
│   │   ├── Cargo.toml
│   │   └── src/main.rs            # currently a stub
│   └── capcom-arch-tests/         # cargo-test architecture gate
├── docs/
│   ├── README.md                  # docs index
│   ├── design/data-model.md       # APH-DM-001 design + diagrams
│   └── APH-DM-001-preflight.md    # codex architecture preflight note
├── scripts/
│   └── check-adr-conformance.sh   # grep-based ADR enforcement
├── .github/workflows/ci.yml       # adr-conformance / lint / test jobs
├── .ground-control.yaml           # Ground Control workflow config for this repo
└── .claude/hooks/adr-boundary-check.sh   # PreToolUse hook for Claude Code
```

## Kernel surface

### Data model

The `data_model` module satisfies `APH-DM-001` (labeled property graph)
with `Node`, `Relationship`, `Label`, `RelationshipType`,
`PropertyKey`/`PropertyValue`/`PropertyMap`, and an in-memory `Graph`
container. Every type is currently `pub(crate)` per the
[architecture preflight](docs/APH-DM-001-preflight.md): the kernel's
public surface stays at `capcom::version()` until a sibling requirement
needs to expose graph operations across the kernel boundary, at which
point [`crates/capcom-arch-tests/tests/kernel_boundary.rs`](crates/capcom-arch-tests/tests/kernel_boundary.rs)
gets a paired update in the same change. Full design notes and Mermaid
class/sequence diagrams: [`docs/design/data-model.md`](docs/design/data-model.md).

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
