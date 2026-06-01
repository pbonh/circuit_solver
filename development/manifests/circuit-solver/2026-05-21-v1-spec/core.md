---
title: "Core manifest — circuit-solver/2026-05-21-v1-spec"
type: manifest-core
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
description: "Circuit solver v1 release: unified analog, digital, and mixed-signal simulator with PyO3 frontend, russell/faer sparse-direct backends, and Sky130/ASAP7 golden-reference conformance harness."
capabilities:
  - dc-operating-point
  - ac-small-signal
  - transient-time-domain
  - noise-spectral-density
  - mixed-signal-cosim
  - python-frontend
scientia_schema: 1
wiki_snapshot: a6ced3d5270bd07a5fdb7d72187adadcfff2b57b
bundle_version: 0.1.0
created: 2026-05-21
---

## 1 — Domain Framing

The circuit-solver context is the umbrella bounded context for the unified analog, digital, and mixed-signal circuit simulator. It delegates to five sub-contexts:

- **netlist-graph** — circuit structure and connectivity; owns `CircuitGraph`, `Node`, `Branch`, `Element`, `Subcircuit`.
- **device-modeling** — semiconductor device equations and stamps; owns `DeviceModel`, `ModelParameters`, `LinearizedModel`, `ModelLibrary`.
- **numeric-solver** — MNA matrix assembly, Newton-Raphson, sparse direct solve; owns `MNAMatrix`, `SparseMatrix`, `LinearSolver`, `NewtonRaphsonSolver`, `IntegrationMethod`.
- **analysis-orchestration** — DC / AC / transient / noise control loops; owns `DCAnalysis`, `ACAnalysis`, `TransientAnalysis`, `NoiseAnalysis`, `AnalysisResult`, `Sweep`, `Waveform`, `FrequencyResponse`.
- **application-frontend** — PyO3 Python API and CLI; owns `CircuitSolverCLI`, `PythonAPI`, `ResultFormatter`, `Session`.

Key invariants: A circuit description entering the system must be parseable into a connected graph. Every analysis request must resolve to a valid control loop with converged solver calls. Results must be immutable and zero-copy across the Python boundary.

Boundary artifacts crossing into adjacent external systems: mixed-signal cosimulation protocol with digital event simulators (iverilog, Verilator).

## 2 — In-Scope Concepts

| Concept | One-line definition |
|---|---|
| [[concepts/modified-nodal-analysis]] | Standard SPICE matrix formulation augmenting node-voltage KCL with branch-current variables for non-admittance elements. |
| [[concepts/newton-raphson-method]] | Iterative root-finding algorithm constructing linear approximations at each iterate until update and residue are below tolerance. |
| [[concepts/device-modeling]] | Electrical behavior of semiconductor devices represented by equation sets, equivalent circuits, or interpolating tables. |
| [[concepts/dc-analysis]] | Steady-state equilibrium computation — node voltages and branch currents satisfying KCL/KVL with zero time derivatives. |
| [[concepts/ac-analysis]] | Sinusoidal small-signal frequency-domain analysis linearized around a DC operating point, reporting magnitude and phase vs. frequency. |
| [[concepts/transient-analysis]] | Time-domain simulation discretizing time, replacing d/dt with finite-difference formulas, and solving nonlinear equations at each timestep. |
| [[concepts/noise-analysis]] | AC-variant computing output spectral density from intrinsic device noise sources linearized around the DC operating point. |
| [[concepts/mixed-level-simulation]] | Simulating one block at transistor level while representing the rest with behavioral pin-accurate models in one concurrent run. |
| [[concepts/golden-reference]] | Precomputed result from an independent trusted tool defining expected output; pass/fail decided by tolerance envelope. |
| [[concepts/event-trace-equivalence]] | Relation holding when two digital runs agree on (time, signal, value) tuples at every cycle boundary, ignoring intra-cycle order. |
| [[concepts/value-change-dump]] | IEEE 1364 ASCII format recording time-stamped value changes of monitored signals during digital simulation. |
| [[concepts/global-interpreter-lock]] | Mutex inside CPython allowing only one native thread to execute bytecode at a time; C extensions can release it around native work. |
| [[concepts/ownership]] | Rust's core memory-management discipline: every value has a single owner, dropped when the owner goes out of scope. |

## 3 — In-Scope Entities

| Entity | One-line overview |
|---|---|
| [[entities/ngspice]] | Open-source descendant of Berkeley SPICE3f5, standard batch-driven analog/mixed-signal simulator. |
| [[entities/icarus-verilog]] | Open-source Verilog compilation/simulation toolchain producing VCD traces via the `vvp` runtime. |
| [[entities/sky130-pdk]] | Open-source 130 nm process design kit with device models and standard-cell libraries under permissive licences. |
| [[entities/asap7-pdk]] | Predictive 7 nm FinFET PDK for academic research; digital-only in v1 per ADR-0005. |
| [[entities/pyo3]] | Canonical Rust crate for native CPython extensions enforcing GIL safety via `Python<'py>` token. |
| [[entities/russell]] | Open-source Rust scientific-computing toolkit; v1 real-valued sparse-direct-LU backend for DC and transient. |
| [[entities/faer]] | Open-source pure-Rust linear-algebra crate; v1 complex-valued sparse-direct-LU backend for AC small-signal. |

## 4 — Ubiquitous Language

- `Circuit` — the top-level object representing a netlist and its associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a specific simulation type requested by the user (DC, AC, transient, noise).
- `Netlist` — the textual or programmatic circuit description.
- `Result` — the unified output structure for any analysis.
- `Golden Reference` — a trusted external simulator against which results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a golden reference.
- `OperatingPoint` — the DC steady-state solution used as a reference for AC/noise/transient.
- `Sweep` — a sequence of analysis points (voltage, frequency, or time).
- `Waveform` — a time-domain voltage or current signal.
- `TransferFunction` — the complex ratio of output to input in AC analysis.
- `SmallSignal` — the linearized behavior around an operating point.
- `LargeSignal` — the full nonlinear time-domain behavior.
- `Convergence` — success or failure of the overall analysis or per-iteration solve.
- `UIC` — Use Initial Conditions, bypassing the DC operating-point calculation.

## False Cognates with Adjacent Contexts

- **Node** — In `netlist-graph`, any electrical vertex. In `numeric-solver`, a matrix row after ground suppression and MNA augmentation; sizes may differ.
- **Model** — In `netlist-graph`, a string key on an element. In `device-modeling`, the full constitutive equation set and parameter set.
- **Operating Point** — In `analysis-orchestration`, the global solution vector. In `device-modeling`, the local terminal bias of one device.
- **Convergence** — In `analysis-orchestration`, overall analysis success. In `numeric-solver`, per-Newton-iteration update/residual criterion.

## 7 — Related Prior Work

| Summary | Relevance |
|---|---|
| [[summaries/computer-methods-circuit-analysis-design-02-motivation]] | Motivation for computer-aided circuit analysis and design. |
| [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]] | Fundamental circuit concepts (KCL, KVL, network graphs). |
| [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]] | Network equation formulation and direct solution methods. |
| [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]] | Graph-theoretic MNA and sparse matrix formulations. |
| [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]] | DC operating-point computation and Newton-Raphson convergence. |
| [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]] | Transient integration methods (Backward Euler, BDF) and DAE handling. |
| [[summaries/graphs-in-vlsi-08-5-circuit-analysis]] | Graph-based circuit analysis formulations and power-grid applications. |
