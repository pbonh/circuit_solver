---
title: "Spec-stage manifest extension — circuit-solver/2026-05-21-v1-spec"
type: manifest-extension
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
stage: spec
scientia_schema: 1
created: 2026-05-21
---

## 9 — Tradeoffs & Suggestions (computed at spec stage)

### Tradeoffs surfaced during spec authoring

| Tradeoff | Chosen | Alternative | Rationale |
|---|---|---|---|
| Scenario-outline vs. example-based | Example-based (concrete values) | Scenario outlines with Examples tables | v1 specs prioritize readability and conformance-test traceability; scenario outlines can be introduced at design stage when parameterized coverage is needed |
| OperatingPoint auto-computation vs. explicit pre-request | Auto-compute when absent | Require explicit DC request before AC/noise | Auto-computation reduces user friction in Python workflows; explicit mode remains available for advanced control |
| Per-device noise breakdown vs. total-only | Per-device breakdown optional in Result | Always include per-device breakdown | Per-device breakdown is valuable for design insight but adds storage and compute; made optional to avoid penalizing bulk sweeps |
| Immutable-handle error vs. silent no-op on CircuitGraph mutation | Raise `ImmutableHandleError` | Return None or silently ignore | Explicit error prevents subtle bugs where user believes mutation succeeded; aligns with Rust's ownership philosophy per ADR-0001 |
| Rollback diagnostic logging vs. silent rollback | Log rollback with diagnostic metadata | Silent rollback | Rollback is architecturally significant (ADR-0004); diagnostics enable performance tuning and correctness auditing |

### Suggestions for downstream stages

- **Design stage** should resolve the exact tolerance-envelope formulation for golden-reference conformance (absolute vs. relative, per-node vs. global), currently an open question in the proposal.
- **Design stage** should specify the analog-digital boundary interpolation scheme (zero-order hold vs. linear) when the analog timestep does not land exactly on a digital event time.
- **Tasks stage** should produce a dedicated conformance-harness task per analysis type (DC, AC, transient, noise, mixed-signal) with PDK-specific test benches.
