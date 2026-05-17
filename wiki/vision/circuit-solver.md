---
title: "Circuit Solver"
type: vision
tags: [circuit-solver, analog, mixed-signal, simulation, rust, python]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "concepts/modified-nodal-analysis"
  - "concepts/newton-raphson-method"
  - "concepts/device-modeling"
  - "concepts/spice"
confidence: high
---

## Value Proposition

This R&D effort delivers a unified analog, digital, and mixed-signal circuit simulator implemented in Rust and Python. Its differentiating property is a memory-safe, graph-native core that represents netlists as typed graphs, assembles and solves sparse MNA systems with modern direct linear algebra, and returns a unified result structure across DC, AC, transient, and noise analyses. Unlike legacy SPICE derivatives written in C, the Rust core eliminates entire classes of memory-safety bugs while providing ergonomic Python bindings for interactive design exploration and batch validation.

## In Scope

- Analog DC operating-point, AC small-signal, transient time-domain, and noise analyses
- Netlist parsing and circuit-graph construction from SPICE-style deck input
- Sparse modified-nodal-analysis (MNA) matrix assembly and direct LU solution
- Newton-Raphson nonlinear solver with homotopy continuation aids (source stepping, Gmin stepping, pseudo-transient)
- Implicit integration methods for transient analysis (Backward Euler, Trapezoidal, Gear BDF)
- Core semiconductor device models: diode, BJT (Ebers-Moll / Gummel-Poon), MOSFET (Level-1 through BSIM4-level)
- Python frontend (`python -m circuit_solver`) and programmatic API
- Unified simulation-result structure (node voltages, branch currents, waveforms, frequency responses)
- Mixed-signal co-simulation hooks (event-driven digital kernel interfaced to continuous-time analog solver)

## Out of Scope

- RF periodic steady-state, harmonic balance, or PAC/PNoise analyses
- Full Verilog-AMS / VHDL-AMS compiler and behavioral analog-HDL execution
- Foundry PDK integration and proprietary model encryption
- GUI schematic capture or layout-aware parasitic extraction
- GPU-accelerated matrix factorization or Monte-Carlo sampling
- Symbolic analysis engine (DDD/GPDD) — kept as a future adjacent effort, not part of the numeric solver core
- Manufacturing variation / corner analysis beyond simple parameter sweep

## Differentiation

- **Memory safety**: Rust's [[concepts/ownership]] and borrowing rules eliminate use-after-free, double-free, and data races that plague C-based SPICE forks.
- **Unified graph view**: The netlist is a first-class [[concepts/graph]] throughout the pipeline, not flattened into ad-hoc arrays, enabling cleaner subcircuit expansion, connectivity queries, and future partitioning.
- **Modern sparse direct solver**: Leverages contemporary sparse-matrix crates and ordering algorithms rather than 1980s-era sparse-matrix code.
- **Python-native ergonomics**: Interactive Python API with NumPy-compatible result arrays, unlike the rigid text-deck batch model of traditional [[entities/spice]].
- **Open and inspectable**: All solver internals (Jacobian, timestep decisions, convergence history) are programmatically accessible for debugging and research, unlike closed-source [[entities/spectre]].

## Revisions

- 2026-05-17 — Initial scope declaration for circuit-solver R&D effort.

## Grill Notes

- [[grills/circuit-solver]] — Open design questions being surfaced and resolved.
