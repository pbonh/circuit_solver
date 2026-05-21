---
title: "Deliver circuit-solver v1: unified analog / digital / mixed-signal simulator with memory-safe Rust core and PyO3 frontend"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
manifest_core: development/manifests/circuit-solver/2026-05-21-v1-spec/core.md
created: 2026-05-21
---

# Proposal: Deliver circuit-solver v1

## Why

The circuit-simulator landscape is dominated by C-code derivatives of Berkeley SPICE3f5 — codebases where memory-safety bugs, undefined behavior, and data races are endemic and costly to debug. At the same time, modern analog / mixed-signal design increasingly demands interactive exploration (Python notebooks, parameter sweeps, algorithmic optimization) rather than rigid batch-deck workflows. A simulator that pairs Rust's ownership discipline with an ergonomic Python API can eliminate whole classes of runtime defects while preserving the numerical fidelity engineers expect.

This change delivers the **v1 release** of `circuit-solver`, a unified analog, digital, and mixed-signal simulator built on five bounded contexts — `netlist-graph`, `device-modeling`, `numeric-solver`, `analysis-orchestration`, and `application-frontend` — that together cover the full simulation pipeline from netlist parsing to result formatting. The Rust core implements [[concepts/modified-nodal-analysis]] (MNA) matrix assembly, [[concepts/newton-raphson-method]] nonlinear iteration, and implicit time integration, backed by modern pure-Rust sparse-direct linear algebra rather than 1980s-era C matrix code. The PyO3 frontend exposes incremental circuit construction and zero-copy NumPy result arrays to Python.

The v1 scope is deliberately bounded to the four canonical analog analyses (DC operating point, AC small signal, transient time domain, and noise spectral density) plus mixed-signal co-simulation hooks against an external event-driven digital simulator. This boundary lets the team validate the architecture end-to-end against open-source golden references — [[entities/sky130-pdk]] and [[entities/asap7-pdk]] — without boiling the ocean on RF steady-state or GUI capture that the [[vision/circuit-solver]] explicitly defers.

Five accepted architectural decisions already constrain the design and are honored by this proposal:
- **ADR-0001** — PyO3 in-process binding with immutable `CircuitGraph` (builder API from Python, zero-copy results).
- **ADR-0002** — Hybrid sparse-direct backend: `russell` for real-valued DC/transient solves, `faer` for complex-valued AC solves.
- **ADR-0003** — Two-pass graph flattening with per-analysis sub-views (one structural flattening, masked solves).
- **ADR-0004** — Optimistic mixed-signal synchronization via a shared scheduler (run-until, sparse checkpoints, rollback on misprediction).
- **ADR-0005** — Closed-enum device-model dispatch (`Diode`, `BJT`, `MOSFET` variants with zero-cost `match`).

## What Changes

- Introduce the `circuit-solver` crate workspace with five bounded-context crates aligned to the container diagram in [[architecture/circuit-solver]].
- Implement SPICE-style netlist parsing, subcircuit expansion, and typed graph construction (`CircuitGraph`) in the `netlist-graph` crate.
- Implement closed-enum device models (diode, BJT Ebers-Moll / Gummel-Poon, MOSFET Level-1 through BSIM4-level) with Jacobian stamp generation in the `device-modeling` crate.
- Implement MNA matrix assembly, Newton-Raphson iteration with source/Gmin stepping homotopy, and hybrid `russell` + `faer` sparse-direct LU dispatch in the `numeric-solver` crate.
- Implement DC operating-point, AC small-signal, transient time-domain (Backward Euler / Trapezoidal / Gear BDF), and noise-spectral-density control loops with adaptive timestepping in the `analysis-orchestration` crate.
- Implement mixed-signal co-simulation hooks: a `MixedSignalScheduler` that issues "run-until" commands to the analog solver, exchanges next-event-time with an external digital simulator (Icarus / Verilator adapters), and performs sparse-checkpoint rollback on misprediction.
- Implement a PyO3 extension module (`circuit_solver`) exposing a builder API for incremental circuit construction, immutable `CircuitGraph` handles, per-request `AnalysisRequest` submission, and NumPy-compatible result arrays.
- Build a conformance harness that compares DC, AC, transient, and noise results against [[entities/ngspice]] golden references using [[entities/sky130-pdk]] and [[entities/asap7-pdk]] test benches, with a tolerance-envelope pass/fail criterion.
- **BREAKING:** The public Rust API surface for `CircuitGraph`, `AnalysisRequest`, and `AnalysisResult` is not yet stabilized; v1.0.0 signals "feature complete for declared scope," not "semver frozen."
- **BREAKING:** Adding a new device-model variant after this change requires editing the closed `DeviceModel` enum and recompiling all downstream `match` sites; there is no runtime plugin mechanism in v1.

## Out of Scope

- RF periodic steady-state, harmonic balance, PAC, or PNoise analyses.
- Full Verilog-AMS / VHDL-AMS compiler or behavioral analog-HDL execution.
- Foundry PDK integration, proprietary model encryption, or runtime device-model loading.
- GUI schematic capture, layout-aware parasitic extraction, or wave-form viewer.
- GPU-accelerated matrix factorization, Monte-Carlo sampling, or symbolic analysis engine.
- Manufacturing-variation / corner analysis beyond simple parameter sweep.
- Incremental netlist modification (adding or removing elements mid-session) without a full graph rebuild.
- Runtime model-parameter override from Python beyond netlist `.model` cards.

## Capabilities Introduced or Modified

- `dc-operating-point` — Steady-state equilibrium computation via Newton-Raphson with homotopy aids; produces node voltages and branch currents.
- `ac-small-signal` — Sinusoidal frequency-domain analysis linearized around a DC operating point; produces magnitude/phase transfer functions via complex-valued sparse LU.
- `transient-time-domain` — Nonlinear time-domain simulation with adaptive timestepping and implicit integration; produces waveforms as time-indexed vectors.
- `noise-spectral-density` — AC-variant computing output-referred noise from intrinsic device noise sources; produces spectral-density vs. frequency.
- `mixed-signal-cosim` — Optimistic time synchronization between continuous-time analog solver and external event-driven digital simulator via shared scheduler.
- `python-frontend` — PyO3 builder API, immutable `CircuitGraph` handles, `AnalysisRequest` submission, and zero-copy NumPy result arrays.

## Open Questions

- What is the exact tolerance envelope for golden-reference conformance — absolute vs. relative, per-node vs. global, and how do we handle nodes that ngspice labels differently after ground suppression?
- Should the PyO3 extension be distributed as a `maturin` / `pip` wheel, a `conda` package, or both? (Build-system decision deferred to implementation tasks.)
- How does the mixed-signal scheduler behave when the external digital kernel violates the next-event-time contract without warning?
- At what circuit scale does sparse-checkpoint memory overhead become prohibitive, and should we fall back to lockstep synchronization above that threshold?

## References

- Manifest core: `development/manifests/circuit-solver/2026-05-21-v1-spec/core.md`
- Relevant ADRs:
  - [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]]
  - [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]]
  - [[decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views|ADR-0003]]
  - [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]]
  - [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]]
- Related concepts:
  - [[concepts/modified-nodal-analysis]]
  - [[concepts/newton-raphson-method]]
  - [[concepts/device-modeling]]
  - [[concepts/dc-analysis]]
  - [[concepts/ac-analysis]]
  - [[concepts/transient-analysis]]
  - [[concepts/noise-analysis]]
  - [[concepts/mixed-level-simulation]]
  - [[concepts/golden-reference]]
  - [[concepts/global-interpreter-lock]]
  - [[concepts/ownership]]
- Related entities:
  - [[entities/ngspice]]
  - [[entities/icarus-verilog]]
  - [[entities/sky130-pdk]]
  - [[entities/asap7-pdk]]
  - [[entities/pyo3]]
  - [[entities/russell]]
  - [[entities/faer]]
- Vision: [[vision/circuit-solver]]
- Architecture: [[architecture/circuit-solver]]
