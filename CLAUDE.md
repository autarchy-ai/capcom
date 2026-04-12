See [AGENTS.md](AGENTS.md) for Ground Control integration, development standards, and repository structure. Everything in AGENTS.md applies here.

Always use the package manager to install dependencies.
Always follow the coding standards.
Keep docs and ADRs up to date.
Always do the right thing, not the easy thing.

## Project

capcom is the product-owned Rust kernel for Aphelion — a graph database
engine. See ADR-001 in the KeplerOps/aphelion repo for the kernel
selection rationale. V1 is single-node only (ADR-003/005/009).

## Build

Build commands land with the first Cargo workspace scaffolding work
order step. Until then, refer to the commented workflow block in
`.ground-control.yaml` for the planned values.

## Ground Control

The Ground Control project identifier for this repo is `capcom`, set
in `.ground-control.yaml` at repo root. The full workflow config
(including workflow commands, sonarcloud settings, and plan rules) is
read by agents via the `gc_get_repo_ground_control_context` MCP tool.

## Code Review

Don't surface nitpicks about PR titles or descriptions unless they are grossly misleading.

## Implementation

Always check your work against the requirement you are implementing to be sure you have implemented all the functionality described in the requirement.

## Answer Questions

If you are asked a question that you don't know the answer to but you have the means to find the facts, go find the facts and answer the question. You have all the tools at your disposal to answer any of these questions, so use them.
