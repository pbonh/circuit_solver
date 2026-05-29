---
adr: 0006
title: Native Event-Driven Digital Engine
status: accepted
created: 2026-05-28
supersedes: decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler
---

# ADR 0006: Native Event-Driven Digital Engine

## Status

accepted

## Y-Statement

In the context of industrial-strength digital simulation within a mixed-signal solver, facing the IPC cost and loose coupling of co-simulating with an external event-driven simulator, we decided for an in-process native event-driven (DEVS-based) digital kernel and
against external co-simulation with Icarus Verilog (the v1 choice, ADR-0004) and a hybrid native/external split, to achieve single-process performance with no IPC and tight integration with the analog optimistic-rollback scheduler, accepting building and maintaining a digital kernel, and superseding accepted ADR-0004.

## Context

v1 co-simulated the digital domain with an **external** event-driven simulator under an optimistic shared scheduler ([[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]], effective 1.045). The multidomain-solver proposal reopened this for an industrial-strength digital target (grill `oq-native-digital-scope`, `cc-adr0004-external-cosim`). The knowledge graph supports a native engine at high confidence: [[concepts/discrete-event-system-specification]] (effective 0.95) and [[concepts/event-driven-architecture]] (effective 0.95). The operator confirmed the native-engine direction at the `write_design` halt (see `question-for-operator.md`, Q1).

## Decision

Build a **native, in-process event-driven digital kernel** (DEVS-style event queue with delta-cycle settling) as a first-class container. The Mixed-Signal Scheduler issues `run-until` to this kernel in-process (no cross-process IPC). Scope is gate-/logic-level event-driven evaluation; full HDL elaboration is out of scope. [[entities/icarus-verilog]] is retained strictly as the **digital golden reference** for event-trace-equivalence validation, not as the runtime engine. The optimistic checkpoint/rollback mechanism of ADR-0004 is retained, now applied to the native kernel's queue and net state.

## Consequences

Positive: single-process execution, no IPC overhead, and tighter mixed-signal rollback consistency. Negative: the project must build and maintain an event-driven kernel, and this **supersedes accepted ADR-0004** — a durable reversal. The canonical wiki page [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]] should be flipped to `superseded` when this change is implemented; it is left `accepted` until then, as this pipeline produces a proposal, not a merge. Correctness remains externally anchored via the Icarus golden trace.

## Sources

- `design.md` (Native Digital Kernel container; supersession note)
- `specs/digital-engine/spec.md`, `specs/mixed-signal-cosim/spec.md`, `specs/digital-equivalence/spec.md`
- Supersedes [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]] (effective 1.045)
- KG: [[concepts/discrete-event-system-specification]] (0.95), [[concepts/event-driven-architecture]] (0.95)
- Inherited confidence (min rollup): 0.95 — presented recommended-accept; operator-confirmed
