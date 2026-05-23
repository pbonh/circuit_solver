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

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `optimistic-advance-with-correct-prediction`** — task `t_a36ef768` (key `2026-05-21-v1-spec:task-48:ff89c4c7`) merged at `10a0ea1c66972d4b33ea2a75ba4f80cf85188828` by `scientia-integrator`. Verification: `cargo test -p analysis-orchestration` → 259 passed / 0 failed / 1 ignored. Residual risk: Mechanical conflict only (doc-bullet union) resolved by implementer respawn; semantic risk low. Changed files: 2.
