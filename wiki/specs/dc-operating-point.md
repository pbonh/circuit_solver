---
title: "Spec: DC Operating Point"
type: spec
capability: dc-operating-point
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
created: 2026-05-21
---

## Source

This spec is a living-documentation mirror of
[openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md).

See the source file for the authoritative Gherkin scenarios, glossary,
and acceptance criteria.

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `conformance-test-against-ngspice-golden-reference` (ASAP7 variant)** — task `t_04b4e126` (key `t_04b4e126`) merged at `55100e5a3c51e2570e780d9623e2133af5ea8561` by `scientia-integrator`. Verification: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (190 passed, 0 failed), `cargo doc --workspace --no-deps` all clean. Residual risk: None. Changed files: 3.
