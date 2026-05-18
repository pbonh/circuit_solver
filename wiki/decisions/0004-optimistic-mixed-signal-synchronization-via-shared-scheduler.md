---
title: "Optimistic Mixed-Signal Synchronization via Shared Scheduler"
type: decision
tags: [decision, circuit-solver, mixed-signal, synchronization, optimistic, scheduler, rollback, checkpoint]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "architecture/circuit-solver"
  - "grills/circuit-solver"
  - "vision/circuit-solver"
  - "contexts/analysis-orchestration"
  - "concepts/mixed-level-simulation"
confidence: high
---

"In the context of mixed-signal co-simulation between a continuous-time analog solver and an external event-driven digital simulator, facing the need for efficient time synchronization with rollback on digital mispredictions while keeping the analog and digital contexts decoupled, we decided for an optimistic time-advance strategy mediated by a shared Mixed-Signal Scheduler that owns both kernels and issues run-until commands, to achieve maximal analog solver efficiency via adaptive timestepping and clean context boundaries, accepting that sparse checkpointing at predicted digital event boundaries adds memory overhead and that mispredictions trigger full re-solve from the last good checkpoint."

## Status

proposed

## Context

The simulator must support mixed-signal co-simulation between the continuous-time analog solver and an external event-driven digital simulator, using optimistic time advancement with efficient rollback on digital mispredictions, while keeping the analog [[contexts/analysis-orchestration|analysis-orchestration]] and digital kernel contexts decoupled (neither queries the other directly). This is an [[concepts/architecturally-significant-requirement|architecturally significant requirement]] (ASR) because it constrains the interaction pattern between two independent simulation kernels, dictates the checkpointing strategy for the analog solver, and determines which context owns the synchronization logic.

The [[vision/circuit-solver|Circuit Solver vision]] explicitly bounds scope to mixed-signal co-simulation hooks, requiring event-driven digital kernel interfaced to continuous-time analog solver. The [[grills/circuit-solver|grill Q&A]] explored synchronization alternatives (lockstep, optimistic, quantized breakpoints, event-driven analog) and rollback strategies (full snapshot, sparse checkpointing, incremental delta logging, predictor-corrector), converging on optimistic advancement with sparse checkpointing coordinated by a shared scheduler process.

The [[architecture/circuit-solver|architecture page]] surfaces this decision under `## Decisions Surfaced` as the fourth top-level commitment.

## Decision

We commit to an optimistic mixed-signal synchronization architecture inside the Mixed-Signal Scheduler container:

1. **Optimistic time advance.** The analog solver runs ahead with its native adaptive timestepping, predicting the next digital event boundary. It does not pause at every digital timestep.

2. **Sparse checkpointing at predicted boundaries.** Before crossing a predicted digital event boundary, the scheduler requests a sparse checkpoint from the analog solver — enough state to resume from that time point if the prediction is wrong.

3. **Shared scheduler ownership.** Neither the analog Analysis Orchestrator nor the external digital simulator queries the other directly. The Mixed-Signal Scheduler owns both kernels and issues "run-until" commands to the analog side and "next-event-time" queries to the digital side. All rollback commands flow through the scheduler.

4. **Rollback on misprediction.** When the digital kernel returns an event earlier than the predicted boundary (or state changes invalidate the analog trajectory), the scheduler instructs the analog solver to roll back to the last good checkpoint and re-solve up to the corrected boundary.

This decision keeps the context boundary clean: the analog solver sees only "run until time T" and "rollback to checkpoint C"; the digital kernel sees only "report next event time" and "deliver events." The scheduler is the sole mediator.

## Consequences

**Positive:**
- Maximal analog solver efficiency: adaptive timestepping proceeds uninterrupted between confirmed boundaries, avoiding the overhead of lockstep synchronization.
- Clean decoupling: the analog and digital kernels remain independent bounded contexts with no direct dependency; the scheduler absorbs all coupling complexity.
- Memory-efficient rollback: sparse checkpoints capture only the state needed to restart (typically node voltages and reactive-element companion-model state), not full dense snapshots.
- Natural extension point: the scheduler can later accommodate multiple digital kernels or analog partitions without redesigning the kernel interfaces.

**Negative:**
- Memory overhead of sparse checkpoints scales with the number of reactive elements and the digital event rate; high-event-rate digital simulations can create checkpoint pressure.
- Full re-solve from checkpoint on misprediction wastes analog computation; frequent mispredictions degrade performance below lockstep levels.
- The scheduler is a single point of failure and a potential bottleneck; its correctness is critical because a mis-ordered "run-until" or rollback command corrupts the mixed-signal state.
- Debugging mixed-signal convergence failures requires tracing across three artifacts (analog trajectory, digital event log, scheduler decision sequence), increasing observability cost.

**Neutral:**
- The decision does not prescribe the sparse checkpoint internal format (full node vector vs. delta-encoded vs. hierarchical Schur-complement); that detail is left to the numeric-solver context.
- The external digital simulator must expose a next-event-time API and accept a rollback-to-checkpoint protocol; simulators lacking this API require an adapter outside the scheduler boundary.
- The decision bounds scope to one analog kernel and one digital kernel per scheduler instance; multi-rate or multi-domain extensions are future work.

## Related Decisions

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]] — Preceding ADR on PyO3 binding; the Python frontend may request mixed-signal analyses that exercise this scheduler.
- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Preceding ADR on solver backends; the analog re-solve after rollback uses the same sparse direct solver stack.
- [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views|ADR-0003]] — Preceding ADR on graph flattening; the checkpoint captures state relative to the flattened structure.
- [[architecture/circuit-solver]] — The container diagram that surfaces this decision under `## Decisions Surfaced`.
- [[grills/circuit-solver]] — Q&A log where synchronization and rollback alternatives were interrogated.
- [[vision/circuit-solver]] — Scope declaration that mandates mixed-signal co-simulation hooks.
- [[contexts/analysis-orchestration]] — Bounded context that receives "run-until" and "rollback" commands from the scheduler.
- [[concepts/mixed-level-simulation]] — Concept page covering mixed analog/digital simulation paradigms.
