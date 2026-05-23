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
- **Scenario `digital-simulator-violates-next-event-time-contract`** — task `t_9181dade` (key `mixed-signal-cosim:ADR-0006:digital-simulator-violates-next-event-time-contract:289b2e9b`) merged at `a7206630b042d706cbde51517deb18ae031a60b1` by `scientia-integrator`. Verification: `cargo fmt/build/clippy/test all green (190 unit + 13 integration tests passed)`. Residual risk: Merge is local-only (origin/main not updated). Prior integrator t_c786184c performed the actual merge. Changed files: 0.
- `t_0e49004b` (#51) — mixed-signal analysis control loop — merged at `2b2b44be4973651f0c98c5895e071dc8786dde4e` via `crates/analysis-orchestration/tests/scenario_mixed_signal_analysis_control_loop_item_51.rs` (test). ADRs: `ADR-0004`, `ADR-0010`. Preflights: fmt OK, clippy OK, test 277 pass/0 fail/1 ignore.
