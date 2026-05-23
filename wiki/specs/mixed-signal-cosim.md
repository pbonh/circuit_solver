---
title: "Spec: Mixed-Signal Co-Simulation"
type: spec
capability: mixed-signal-cosim
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
created: 2026-05-21
---

## Source

This spec is a living-documentation mirror of
[openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/mixed-signal-cosim/spec.md](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/mixed-signal-cosim/spec.md).

See the source file for the authoritative Gherkin scenarios, glossary,
and acceptance criteria.

## Implementation Evidence

This section is a living-documentation mirror of the source spec.
Updates are managed by `scientia-ingest-evidence`; do not edit manually.

<!-- scientia-ingest-evidence-keyed: 2026-05-21-v1-spec:task-67:01133d40ad785833eba58aa14549fb0054ad077aacb1d3d5beb87860a765a47f -->
- **Scenario `mixed-signal-conformance-with-event-trace-equivalence`** — task `t_c951701f` (key `2026-05-21-v1-spec:task-67:01133d40ad785833eba58aa14549fb0054ad077aacb1d3d5beb87860a765a47f`) merged at `2b0a5342f77486347bf6288fe8f1c2ec3b98bf94` by `scientia-integrator`. Verification: `cargo test --workspace` all pass (196+ tests); `cargo fmt` clean; `cargo clippy` clean; 4 integration tests pass. Residual risk: digital event-trace equivalence currently compares SimulationTime values only; analog golden synthesized from closed-form (same pattern as DC/transient/noise tests); rollback paths not exercised. Changed files: 1.

<!-- scientia-ingest-evidence-keyed: 2026-05-21-v1-spec:t_1dd43f00:17cb7f610f24b7d56ce96f58858f18b2e227a694 -->
- **Scenario `optimistic-advance-with-correct-prediction`** — task `t_1dd43f00` (key `2026-05-21-v1-spec:t_1dd43f00:17cb7f610f24b7d56ce96f58858f18b2e227a694`) merged at `17cb7f610f24b7d56ce96f58858f18b2e227a694` by `scientia-integrator` (no-op pass-through; originally merged by `t_e202ff6a`). Verification: `cargo fmt` clean; `cargo clippy` clean; `cargo test --workspace` all pass (196+ tests). Residual risk: stubbed rollback paths not exercised by this scenario. Changed files: 18.

<!-- scientia-ingest-evidence-keyed: 2026-05-21-v1-spec:t_3071313c:e8263533d9e2e3720814a76e604d0bc70008e0c8ff6ad9d91e1eb09379b45552 -->
- **Scenario `mixed-signal-conformance-with-event-trace-equivalence`** — task `t_3071313c` (key `2026-05-21-v1-spec:t_3071313c:e8263533d9e2e3720814a76e604d0bc70008e0c8ff6ad9d91e1eb09379b45552`) merged at `2b0a5342f77486347bf6288fe8f1c2ec3b98bf94` by `scientia-integrator`. No-op pass-through; work already on main from prior integrator `t_c951701f`. Verification: `cargo fmt` clean; `cargo clippy` clean; `cargo test --workspace` all pass (196+ tests); 4 mixed-signal conformance integration tests pass. Residual risk: none known. Changed files: none.
