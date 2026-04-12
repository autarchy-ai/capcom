//! Capcom engine node process.
//!
//! Per ADR-012, capcom runs as a co-located out-of-process engine node
//! alongside the Aphelion product server. This binary is the entry
//! point for that process. The product/kernel IPC contract (ADR-014)
//! will be implemented in a later work order step.

fn main() {
    println!("capcom engine node v{}", capcom::version());
    println!("not yet implemented -- see aphelion/notes/work-order.md");
}
