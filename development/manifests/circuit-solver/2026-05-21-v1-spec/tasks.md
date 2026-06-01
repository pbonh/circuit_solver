---
title: "Tasks manifest — circuit-solver/2026-05-21-v1-spec"
type: manifest-tasks
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
scientia_schema: 1
wiki_snapshot: 6020b5ad53b46efc6e30eee129a14e7a852d8f65
created: 2025-07-18
---

## 9 — Tradeoffs & Suggestions (computed at tasks stage)

### INVEST rules constraining decomposition

| INVEST Property | Application |
|---|---|
| **Independent** | Tasks within a capability are ordered by dependency, but cross-capability tasks (e.g., workspace scaffold, PyO3 binding) are shared and must not duplicate. A task references its dependency by number rather than re-implementing it. |
| **Negotiable** | Each task names the spec scenario or ADR it serves; if a scenario's scope changes, only the tasks tagged with that scenario need revision. |
| **Valuable** | Every task produces one observable output (a struct, a test, a passing scenario). No "set up meeting" or "investigate" tasks — investigation outcomes are recorded as decisions first. |
| **Estimable** | Tasks are scoped to a single coding session. If a task would take more than one session, it is split. |
| **Small** | Max one crate-public API surface per task. Internal helpers are inlined. |
| **Testable** | Every behavioral task names a spec scenario; every ADR task has an acceptance criterion derived from the ADR consequences. |

### Story-splitting heuristics applied

- **Workflow step split**: DC analysis is split into graph-flatten → stamp → NR-solve → convergence-check because each step has a distinct output and can be tested in isolation.
- **Data-variation split**: Device models are split by family (Diode, BJT, MOSFET) because each has independent parameter sets and stamp equations.
- **Interface split**: PyO3 binding is split into builder API, immutable handle, analysis request, and result array because each exercises a different PyO3 interop pattern.

### Suggestions for downstream stages

- **Verify stage** should confirm that every spec scenario is reachable from at least one task.
- **Kanban emit** should produce one Gherkin-scenario task per spec scenario; the `tasks.md` checklist is inlined into the kanban parent as a progress tracker.
