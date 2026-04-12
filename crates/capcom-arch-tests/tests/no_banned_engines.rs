//! ADR-001: capcom is the product-owned engine kernel for Aphelion.
//! The alternative engines evaluated and rejected by that ADR must not
//! be referenced anywhere under `crates/`.
//!
//! This test walks every `.rs` and `Cargo.toml` file under `crates/`
//! (excluding this crate, which holds the banned list as data by design)
//! and fails if any banned engine name appears, even in comments -- the
//! point is that we don't want these names anywhere in the kernel.
//!
//! Lines carrying `// adr-override: ADR-001` are permitted as a
//! deliberate escape hatch and must include a rationale.

use std::fs;

use capcom_arch_tests::{collect_workspace_sources, has_override, workspace_relative};

const BANNED_ENGINES: &[&str] = &["millenniumdb", "graphflow"];

const ENGINE_OVERRIDE_ADRS: &[&str] = &["ADR-001"];

#[test]
fn no_banned_engines_in_workspace() {
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
            let lower = line.to_lowercase();
            if has_override(&lower, ENGINE_OVERRIDE_ADRS) {
                continue;
            }
            for pattern in BANNED_ENGINES {
                if lower.contains(pattern) {
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
        "ADR-001 violation -- banned graph engine references detected:\n{}\n\n\
         ADR-001 selected capcom as the product-owned engine kernel.\n\
         The alternatives were evaluated and rejected.",
        violations.join("\n")
    );
}
