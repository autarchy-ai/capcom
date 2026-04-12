#!/usr/bin/env bash
# ADR Conformance Check -- pre-commit hook and CI script for capcom.
# Scans Rust source and Cargo manifests for ADR violations.
# adr-override: ADR-003 -- This file defines the search patterns, not distributed code.
#
# Enforcement (aphelion ADRs that apply to the capcom kernel):
#   ADR-001: No banned graph engines (MillenniumDB, Graphflow)
#   ADR-003, 005, 009: No distributed systems primitives in V1
#
# Usage:
#   As pre-commit hook: runs against staged files
#   As CI script: runs against all source files under crates/
#
# Override: Lines containing "# adr-override: ADR-NNN" (or "// adr-override:")
# are excluded from the matching checks. Overrides must include a rationale.

set -euo pipefail

ERRORS=0

# Determine file list: staged files (pre-commit) or all source files (CI)
if git rev-parse --is-inside-work-tree &>/dev/null && [ -n "$(git diff --cached --name-only 2>/dev/null)" ]; then
  # Pre-commit mode: check staged files
  FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(rs|toml)$' || true)
else
  # CI mode: check all source files under crates/
  FILES=$(find crates -type f \( -name '*.rs' -o -name 'Cargo.toml' \) 2>/dev/null || true)
fi

if [ -z "$FILES" ]; then
  exit 0
fi

# ---------------------------------------------------------------------------
# CHECK 1: Banned graph engines (ADR-001)
# ---------------------------------------------------------------------------
BANNED_ENGINES='\b(millenniumdb|graphflow)\b'

for file in $FILES; do
  [ -f "$file" ] || continue
  MATCHES=$(grep -inE "$BANNED_ENGINES" "$file" | grep -iv 'adr-override:.*ADR-001' || true)
  if [ -n "$MATCHES" ]; then
    echo "FAIL (ADR-001): Banned graph engine reference in $file:"
    echo "$MATCHES"
    echo "  ADR-001 selected capcom as the product-owned engine kernel."
    echo "  MillenniumDB and Graphflow were evaluated and rejected."
    echo ""
    ERRORS=$((ERRORS + 1))
  fi
done

# ---------------------------------------------------------------------------
# CHECK 2: Distributed systems primitives (ADR-003, ADR-005, ADR-009)
# Patterns are loaded from a variable to keep them in one place.
# ---------------------------------------------------------------------------
# shellcheck disable=SC2034
DIST_TERMS=(
  raft_consensus paxos_consensus leader_election quorum_vote
  two_phase_commit distributed_txn distributed_transaction
  log_shipping log_shipper multi_primary cluster_coordinator
  node_discovery consensus_protocol replication_manager
  failover_controller shard_manager cross_node_query
)
DISTRIBUTED_PATTERN=$(IFS='|'; echo "${DIST_TERMS[*]}")
DISTRIBUTED_PATTERN="\\b(${DISTRIBUTED_PATTERN})\\b"

for file in $FILES; do
  [ -f "$file" ] || continue
  # Strip Rust comment lines (//, ///, //!, /*, *) and shell comments (#) before checking
  MATCHES=$(grep -nE "$DISTRIBUTED_PATTERN" "$file" | grep -ivE '^[0-9]+:\s*(///|//!|//|/\*|\*|#)' | grep -iv 'adr-override:.*ADR-00[359]' || true)
  if [ -n "$MATCHES" ]; then
    echo "FAIL (ADR-003/005/009): Distributed systems primitive in $file:"
    echo "$MATCHES"
    echo "  V1 is single-node only. Distributed consensus, replication,"
    echo "  failover, cross-node operations, and distributed transactions"
    echo "  are out of scope for the current kernel."
    echo ""
    ERRORS=$((ERRORS + 1))
  fi
done

# ---------------------------------------------------------------------------
# Result
# ---------------------------------------------------------------------------
if [ "$ERRORS" -gt 0 ]; then
  echo "ADR conformance check failed with $ERRORS violation(s)."
  echo "To override a specific check, add a line containing"
  echo "'// adr-override: ADR-NNN -- <rationale>' near the offending code."
  exit 1
fi

exit 0
