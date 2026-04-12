//! ADR-003 / ADR-005 / ADR-009: V1 is single-node only. Distributed
//! consensus, replication, failover, cross-node operations, and
//! distributed transactions are out of scope for the current kernel.
//!
//! This test walks every `.rs` and `Cargo.toml` file under `crates/`
//! (excluding this crate, which holds the banned-pattern list by design)
//! and fails if any of the banned primitives appear in non-comment code.
//!
//! Lines carrying `// adr-override: ADR-003` (or ADR-005 / ADR-009) are
//! permitted -- overrides must still be justified in a rationale comment,
//! but the test respects them as a deliberate escape hatch.

use std::fs;

use capcom_arch_tests::{
    collect_workspace_sources, has_override, is_comment_line, workspace_relative,
};

const BANNED_DISTRIBUTED_PRIMITIVES: &[&str] = &[
    "raft_consensus",
    "paxos_consensus",
    "leader_election",
    "quorum_vote",
    "two_phase_commit",
    "distributed_txn",
    "distributed_transaction",
    "log_shipping",
    "log_shipper",
    "multi_primary",
    "cluster_coordinator",
    "node_discovery",
    "consensus_protocol",
    "replication_manager",
    "failover_controller",
    "shard_manager",
    "cross_node_query",
];

const DISTRIBUTED_OVERRIDE_ADRS: &[&str] = &["ADR-003", "ADR-005", "ADR-009"];

#[test]
fn no_distributed_primitives_in_workspace() {
    let files = collect_workspace_sources();
    assert!(
        !files.is_empty(),
        "expected at least one source file under crates/; the walker found none"
    );

    let mut violations: Vec<String> = Vec::new();
    for file in &files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", file.display()));
        for (idx, line) in content.lines().enumerate() {
            if is_comment_line(line) {
                continue;
            }
            if has_override(line, DISTRIBUTED_OVERRIDE_ADRS) {
                continue;
            }
            for pattern in BANNED_DISTRIBUTED_PRIMITIVES {
                if line.contains(pattern) {
                    violations.push(format!(
                        "{}:{}: matches `{}`: {}",
                        workspace_relative(file),
                        idx + 1,
                        pattern,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "ADR-003/005/009 violation -- distributed systems primitives detected:\n{}\n\n\
         V1 is single-node only. If this is future-scoped work, place it behind \
         a disabled feature gate and add `// adr-override: ADR-003 -- <rationale>` \
         on the offending line.",
        violations.join("\n")
    );
}
