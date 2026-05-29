---
title: Native Event-Driven Digital Engine
type: claim
id: decisions/0006-native-event-driven-digital-engine
tags:
- decision
- circuit-solver
- digital
- event-driven
- devs
- mixed-signal
- native-engine
created: '2026-05-28'
updated: '2026-05-28'
supersedes: decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler
sources:
- architecture/circuit-solver
- grills/circuit-solver
- vision/circuit-solver
- contexts/analysis-orchestration
- concepts/discrete-event-system-specification
- concepts/event-driven-architecture
confidence:
  base: 0.95
  source_count: 6
  contradicted: false
  effective: 1.045
  inputs_hash: 01820d1373ce82bd
---
"In the context of industrial-strength digital simulation within a mixed-signal solver, facing the IPC cost and loose coupling of co-simulating with an external event-driven simulator, we decided for an in-process native event-driven (DEVS-based) digital kernel and against external co-simulation with Icarus Verilog (the v1 choice, ADR-0004) and a hybrid native/external split, to achieve single-process performance with no IPC and tight integration with the analog optimistic-rollback scheduler, accepting that we must build and maintain a digital kernel and that this supersedes accepted ADR-0004."

## Status

accepted

Supersedes [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]].

## Context

v1 co-simulated the digital domain with an **external** event-driven simulator under an optimistic shared scheduler ([[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]]). The multidomain-solver change reopened this for an industrial-strength digital target. The knowledge graph supports a native engine at high confidence: [[concepts/discrete-event-system-specification|DEVS]] and [[concepts/event-driven-architecture|event-driven architecture]]. The operator confirmed the native-engine direction at the design stage (`proposals/2026-05-28-multidomain-solver-architecture/question-for-operator.md`, Q1).

## Decision

Build a **native, in-process event-driven digital kernel** (DEVS-style event queue with delta-cycle settling) as a first-class container. The Mixed-Signal Scheduler issues `run-until` to this kernel in-process (no cross-process IPC). Scope is gate-/logic-level event-driven evaluation; full HDL elaboration is out of scope. [[entities/icarus-verilog]] is retained strictly as the **digital golden reference** for event-trace-equivalence validation, not as the runtime engine. The optimistic checkpoint/rollback mechanism of ADR-0004 is retained, now applied to the native kernel's queue and net state.

## Consequences

**Positive:**
- Single-process execution with no IPC overhead and tighter mixed-signal rollback consistency.

**Negative:**
- The project must build and maintain an event-driven kernel.
- This **supersedes accepted ADR-0004**, a durable reversal of the external-co-simulation decision.

**Neutral:**
- Correctness remains externally anchored: Icarus Verilog stays the digital golden trace for event-trace equivalence.

## Related Decisions

- [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]] — Superseded by this ADR; its optimistic-rollback mechanism is retained over the native kernel.
- [[contexts/analysis-orchestration]] — Bounded context owning the Mixed-Signal Scheduler that drives the native kernel.
- [[concepts/discrete-event-system-specification]] — DEVS formalism underpinning the native kernel.
- [[concepts/event-driven-architecture]] — Event-driven evaluation model.
- [[architecture/circuit-solver]] — Container diagram surfacing the decisions.
- [[grills/circuit-solver]] — Q&A log where the digital boundary was interrogated.
- [[vision/circuit-solver]] — Scope declaration for the mixed-signal hooks.

## Provenance

Recorded by the scientia pipeline for change `2026-05-28-multidomain-solver-architecture` (design halt Q1, operator-confirmed; inherited confidence 0.95, recommended-accept). See `proposals/2026-05-28-multidomain-solver-architecture/`.
