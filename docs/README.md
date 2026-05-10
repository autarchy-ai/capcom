# capcom docs

Capcom-local documentation. Aphelion-wide ADRs (the binding ones — ADR-001
through ADR-014 and beyond) live in
[KeplerOps/aphelion](https://github.com/KeplerOps/aphelion); only kernel-
local design notes and per-requirement preflight notes live here.

## Layout

```
docs/
├── README.md                       # this file
├── design/                         # capcom-local design notes
│   └── data-model.md               # APH-DM-001 labeled property graph
└── APH-*-preflight.md              # per-requirement architecture preflights
```

## Conventions

- Diagrams use Mermaid embedded in Markdown so GitHub renders them with no
  toolchain and so diffs stay reviewable. No `.svg`, `.png`, or `.drawio`
  binaries.
- Per-requirement preflight notes are produced by
  `gc_codex_architecture_preflight` at the start of an implementation work
  order. They are authoritative guardrails for that requirement.
- Capcom does not currently host any local ADRs. The aphelion ADR set is
  authoritative; new local ADRs are only created when implementation work
  exposes a real conflict that aphelion ADRs do not already cover (per
  the `APH-DM-001` preflight).
