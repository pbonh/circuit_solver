---
change-id: 2026-05-28-multidomain-solver-architecture
created: 2026-05-28
---

# Design: Multidomain Circuit Solver Architecture

## Overview

This design extends the accepted v1 architecture to industrial-strength coverage across
**analog** (continuous-time), **digital** (event-driven), and **mixed-signal** domains,
for both the device-modeling and simulation-engine layers. It is a single Rust process
exposing a PyO3 frontend, decomposed into six containers aligned to the wiki's bounded
contexts.

Decisions carried in unchanged (high inherited confidence): in-process PyO3 with an
immutable `CircuitGraph` ([[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]],
effective 1.045); pure-Rust hybrid sparse-direct solver — russell (real) + faer (complex)
([[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer]], 1.045); two-pass
graph flattening with per-analysis sub-views
([[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views]], 1.045).

Decisions changed by this design (operator-confirmed at the design halt; see
`question-for-operator.md`):

- **Native digital engine replaces external co-simulation** — a new *Native Digital
  Kernel* container built on [[concepts/discrete-event-system-specification]] (effective
  0.95) and [[concepts/event-driven-architecture]] (effective 0.95). This **supersedes**
  [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]] (1.045);
  the record-adr stage must emit the superseding ADR. Icarus Verilog is retained as the
  digital **golden reference** only.
- **In-tree codegen seam for device-model families** — refines (does not reverse)
  [[decisions/0005-closed-enum-device-model-dispatch]] (1.045): models remain a closed,
  compile-time-monomorphized `enum DeviceModel`; a macro/codegen seam scales the library
  without runtime registration.

Out of scope (confirmed): steady-state / RF engines (harmonic balance, shooting;
[[concepts/shooting-method]] effective 0.85).

## System Context (C4 L1)

```mermaid
C4Context
    title System Context — Multidomain Circuit Solver
    Person(designer, "Circuit Designer", "Builds circuits and runs analyses from Python")
    System(sys, "circuit-solver", "Pure-Rust analog/digital/mixed-signal simulator with a PyO3 frontend")
    System_Ext(ngspice, "ngspice", "Analog golden reference")
    System_Ext(icarus, "Icarus Verilog", "Digital golden reference (validation only)")
    System_Ext(pdk, "PDKs", "sky130 / asap7 device + cell data")
    Rel(designer, sys, "drives", "PyO3 / Python")
    Rel(sys, ngspice, "validated against")
    Rel(sys, icarus, "validated against")
    Rel(sys, pdk, "reads models / cells from")
```

## Container Diagram (C4 L2)

```mermaid
C4Container
    title Containers — circuit-solver (single Rust process)
    Person(designer, "Circuit Designer")
    System_Ext(ngspice, "ngspice", "Analog golden ref")
    System_Ext(icarus, "Icarus Verilog", "Digital golden trace")
    Container_Boundary(corep, "circuit-solver (single Rust process)") {
        Container(frontend, "Application Frontend", "Rust / PyO3", "Immutable CircuitGraph builder, zero-copy results, GIL release [ADR-0001]")
        Container(netlist, "Netlist Graph", "Rust", "Immutable graph; two-pass flattening, per-analysis sub-views [ADR-0003]")
        Container(orch, "Analysis Orchestration", "Rust", "Mixed-Signal Scheduler: optimistic time advance + checkpoint/rollback")
        Container(numeric, "Numeric Solver", "Rust", "Sparse LU — russell (real DC/transient) + faer (complex AC) [ADR-0002]")
        Container(devices, "Device Model Engine", "Rust", "Closed enum DeviceModel + in-tree codegen seam [ADR-0005, refined]")
        Container(digital, "Native Digital Kernel", "Rust", "Event-driven DEVS engine [supersedes ADR-0004]")
    }
    Rel(designer, frontend, "builds circuits, runs analyses")
    Rel(frontend, netlist, "builds")
    Rel(frontend, orch, "requests analyses")
    Rel(orch, netlist, "flattens / sub-views")
    Rel(orch, numeric, "solves MNA systems")
    Rel(orch, digital, "run-until (in-process)")
    Rel(numeric, devices, "stamps in Newton loop")
    Rel(orch, ngspice, "validated against")
    Rel(digital, icarus, "validated against")
```

## Components & Approach

Six containers, each mapping to a bounded-context entity and the capability specs:

- **Application Frontend** ([[contexts/application-frontend]]) — PyO3 binding; immutable
  `CircuitGraph` builder, zero-copy NumPy result views, GIL release on long solves.
  Spec: `frontend-contract`. Honors ADR-0001.
- **Netlist Graph** ([[contexts/netlist-graph]]) — owns the immutable graph and the
  two-pass flatten producing per-analysis sub-views. Honors ADR-0003.
- **Analysis Orchestration** ([[contexts/analysis-orchestration]]) — the Mixed-Signal
  Scheduler: drives DC/AC/transient/noise analyses and the optimistic analog↔digital
  time advance with checkpoint/rollback. Specs: `analog-engine`, `mixed-signal-cosim`.
- **Numeric Solver** ([[contexts/numeric-solver]]) — [[concepts/modified-nodal-analysis]]
  (0.95) assembly + [[concepts/newton-raphson-method]] (0.95) with
  [[concepts/gmin-stepping]]/[[concepts/source-stepping]] (0.95) continuation;
  [[concepts/backward-euler]] (0.988)/[[concepts/trapezoidal-rule]] integration; sparse LU
  via russell/faer. Spec: `analog-engine`. Honors ADR-0002.
- **Device Model Engine** ([[contexts/device-modeling]]) — closed `enum DeviceModel` with
  static dispatch; the new **codegen seam** generates model-family variants at compile
  time. Spec: `device-modeling`. ADR-0005, refined.
- **Native Digital Kernel** — in-process event-driven engine (delta-cycle settling,
  ordered event queue). Specs: `digital-engine`, `digital-equivalence`. Supersedes ADR-0004.

### Device Model Engine internals (C4 L3 — the codegen seam is a new internal)

```mermaid
C4Component
    title Components — Device Model Engine
    Container(numeric, "Numeric Solver", "Rust", "Newton-Raphson loop")
    Component(model_enum, "enum DeviceModel", "Rust", "Closed, monomorphized dispatch")
    Component(codegen, "Model codegen seam", "Rust macro", "Generates model-family variants at compile time")
    Component(stamp, "Stamp evaluator", "Rust", "Per-variant Jacobian + RHS contributions")
    Rel(numeric, stamp, "calls per device")
    Rel(stamp, model_enum, "matches variant")
    Rel(codegen, model_enum, "generates variants into")
```

## Trade-offs

- **Native digital kernel vs. external co-simulation.** Chosen: native (single process, no
  IPC, tighter mixed-signal rollback). Accepted cost: building and maintaining an
  event-driven kernel, and **superseding an accepted ADR (0004)** — a durable reversal.
  Icarus is kept as golden reference, so correctness is still externally anchored.
- **Codegen seam vs. hand-written enum variants.** Chosen: compile-time macro seam.
  Accepted cost: macro/build complexity, in exchange for scaling the model library while
  keeping zero-cost dispatch and rejecting runtime registration (ADR-0005 invariant intact).
- **Pure-Rust solver ceiling.** No C/C++ fallback (ADR-0002). Risk: russell/faer behavior
  at industrial matrix sizes/conditioning — to be answered with benchmarks, not FFI
  (grill `cc-adr0002-pure-rust`).
- **Optimistic sync overhead.** Checkpoint memory + re-solve on digital misprediction
  (ADR-0004 mechanism retained under the native kernel).
- **Steady-state deferral.** Keeps the spec surface bounded; RF/PSS users unserved for now.
- **Event-trace (not byte-VCD) equivalence** as the digital metric, defined operationally
  in `digital-equivalence` so acceptance does not rest on the 0.70 stub claims
  (grill `ha-event-trace-equivalence`, `ha-value-change-dump`).
