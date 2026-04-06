# Agent Instructions

This file provides context for AI coding agents (Claude Code, Codex, Copilot, etc.) working in this repository.

## Ground Control

This project uses **Ground Control** for requirements management and traceability. Ground Control is a requirements engineering platform that stores requirements, relations (parent/child, dependencies), traceability links (code ↔ requirements), ADRs, and project status.

### How it works

- **Requirements** have a UID (e.g. `REQ-001`), title, statement, rationale, type, priority (MoSCoW), status (DRAFT → ACTIVE → DEPRECATED), and wave number.
- **Relations** connect requirements: PARENT (decomposition), DEPENDS_ON, REFINES, RELATED.
- **Traceability links** connect requirements to artifacts: IMPLEMENTS (source files), TESTS (test files), GITHUB_ISSUE (issues).
- **ADRs** (Architecture Decision Records) capture design decisions linked to requirements.

### Integration

Ground Control is available as an MCP (Model Context Protocol) server configured in `.mcp.json`. If your agent supports MCP, the following tools are available:

| Tool | Purpose |
|------|---------|
| `gc_get_requirement` | Fetch a requirement by UID |
| `gc_list_requirements` | List/search requirements with filters |
| `gc_get_relations` | Get parent/child and dependency relations |
| `gc_get_traceability` | Get traceability links for a requirement |
| `gc_create_traceability_link` | Link a requirement to a code file, test, or issue |
| `gc_transition_status` | Move a requirement from DRAFT → ACTIVE |
| `gc_create_github_issue` | Create a GitHub issue from a requirement |
| `gc_list_adrs` | List architecture decision records |

The Ground Control project identifier for this repo is set in `CLAUDE.md`. Use it in the `project` parameter for all GC tool calls.

### Workflow

When implementing a feature tied to a requirement:

1. **Fetch the requirement** — read its statement, rationale, and acceptance criteria.
2. **Check existing traceability** — see what's already implemented and tested.
3. **Implement** — satisfy every clause in the requirement statement.
4. **Create traceability links** — IMPLEMENTS links to source files, TESTS links to test files.
5. **Transition status** — move DRAFT requirements to ACTIVE once implemented.

If your agent does not support MCP, you can still follow this workflow manually by referencing requirement UIDs in commit messages and PR descriptions.

## Development Standards

- Write tests for significant behaviors.
- Document architectural decisions in `docs/adrs/` using MADR format.
- Update `CHANGELOG.md` for every commit that changes source code (not required for documentation-only or GC-only changes).
- Never include AI attribution (Co-Authored-By, "Generated with Claude Code", etc.) in commits or PRs.
- Never merge PRs — the maintainer reviews and merges.

## Repository Structure

```
.claude/          # Claude Code agent configuration (settings, hooks, rules, skills)
.github/          # GitHub Actions CI/CD workflows
docs/adrs/        # Architecture Decision Records (when created)
```
