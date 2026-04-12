#!/usr/bin/env bash
# ADR Boundary Check -- PreToolUse hook for Edit/Write operations.
# Blocks code that violates architectural decisions that apply to the capcom kernel.
#
# Enforcement (aphelion ADRs that apply to capcom):
#   ADR-001: No banned graph engines (MillenniumDB, Graphflow)
#   ADR-003, 005, 009: No distributed systems primitives in V1
#
# Override: Include "// adr-override: ADR-NNN" in the content to bypass
# a specific check. Overrides must include a rationale comment.
#
# capcom IS the kernel, so the aphelion layer boundaries (domain/api vs
# infrastructure/engine) do not apply here. This hook enforces only the
# cross-cutting invariants that hold at every layer of the kernel.

set -euo pipefail

# Read tool input from stdin (JSON with file_path + new_string or content)
TOOL_INPUT=$(cat)

FILE_PATH=$(echo "$TOOL_INPUT" | jq -r '.tool_input.file_path // empty')
if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Extract the content being written -- Edit uses new_string, Write uses content
CONTENT=$(echo "$TOOL_INPUT" | jq -r '.tool_input.new_string // .tool_input.content // empty')
if [[ -z "$CONTENT" ]]; then
  exit 0
fi

# Skip non-source files (docs, configs, this hook itself, lockfiles, manifests)
# and skip the architecture-tests crate, which contains the banned patterns
# as literal strings by design in order to enforce them.
case "$FILE_PATH" in
  */docs/*|*/.claude/*|*/.github/*|*/scripts/*|*/crates/capcom-arch-tests/*|*.md|*.yaml|*.yml|*.json|*.toml|*.lock|*.txt|*.sh)
    exit 0
    ;;
esac

# ---------------------------------------------------------------------------
# Helper: check if content contains an override for the given ADR
# ---------------------------------------------------------------------------
has_override() {
  local adr="$1"
  echo "$CONTENT" | grep -qi "adr-override:.*${adr}" && return 0
  return 1
}

# ---------------------------------------------------------------------------
# CHECK 1: Banned graph engines (ADR-001)
# ---------------------------------------------------------------------------
if echo "$CONTENT" | grep -qiE '\b(millenniumdb|graphflow)\b'; then
  if ! has_override "ADR-001"; then
    echo "BLOCKED (ADR-001): Reference to a banned graph engine detected." >&2
    echo "ADR-001 selected capcom as the product-owned engine kernel." >&2
    echo "MillenniumDB (GPL, not production-ready) and Graphflow (research artifact) were rejected." >&2
    echo "To override, add: // adr-override: ADR-001 -- <rationale>" >&2
    exit 2
  fi
fi

# ---------------------------------------------------------------------------
# CHECK 2: Distributed systems primitives (ADR-003, ADR-005, ADR-009)
# ---------------------------------------------------------------------------
# Match significant usage only -- strip Rust comment lines first.
DISTRIBUTED_PATTERN='\b(raft_consensus|paxos_consensus|leader_election|quorum_vote|two_phase_commit|distributed_txn|distributed_transaction|log_shipping|log_shipper|multi_primary|cluster_coordinator|node_discovery|consensus_protocol|replication_manager|failover_controller|shard_manager|cross_node_query)\b'

if echo "$CONTENT" | grep -qiE "$DISTRIBUTED_PATTERN"; then
  # Strip Rust comment lines (//, ///, //!, /*, *) before re-checking
  STRIPPED=$(echo "$CONTENT" | grep -viE '^\s*(///|//!|//|/\*|\*)')
  if echo "$STRIPPED" | grep -qiE "$DISTRIBUTED_PATTERN"; then
    if ! has_override "ADR-003" && ! has_override "ADR-005" && ! has_override "ADR-009"; then
      echo "BLOCKED (ADR-003/ADR-005/ADR-009): Distributed systems primitive detected." >&2
      echo "V1 is single-node only. Distributed consensus, replication, failover, and" >&2
      echo "cross-node operations are out of scope for the current kernel." >&2
      echo "If this is future-scoped work, place it behind a disabled feature gate." >&2
      echo "To override, add: // adr-override: ADR-003 -- <rationale>" >&2
      exit 2
    fi
  fi
fi

# All checks passed
exit 0
