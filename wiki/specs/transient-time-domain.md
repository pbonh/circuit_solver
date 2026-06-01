---
title: "Spec: Transient Time Domain"
type: spec
capability: transient-time-domain
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
created: 2026-05-21
---

## Source

This spec is a living-documentation mirror of
[openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/transient-time-domain/spec.md](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/transient-time-domain/spec.md).

See the source file for the authoritative Gherkin scenarios, glossary,
and acceptance criteria.

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `transient-conformance-against-ngspice`** — task `t_e81eee59` (key `t_e81eee59`) merged at `42e2cf90c1c7285cc39556ae05bcb3cfb5615b78` by `scientia-integrator`. Verification: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (190 passed, 0 failed, 2 scenario tests passed), `cargo doc --workspace --no-deps` all clean. Residual risk: None. Changed files: 6.
- **Scenario `transient-conformance-against-ngspice` (ASAP7 variant)** — task `t_04b4e126` (key `t_04b4e126`) merged at `55100e5a3c51e2570e780d9623e2133af5ea8561` by `scientia-integrator`. Verification: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (190 passed, 0 failed), `cargo doc --workspace --no-deps` all clean. Residual risk: None. Changed files: 3.
