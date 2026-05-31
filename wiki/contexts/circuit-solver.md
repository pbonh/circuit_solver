---
title: "Circuit Solver"
type: context
tags: [circuit-solver, bounded-context, umbrella]
created: 2026-05-21
updated: 2026-05-21
sources:
  - "vision/circuit-solver"
  - "architecture/circuit-solver"
confidence: high
---

## Model

The circuit-solver context is the umbrella bounded context for the unified analog, digital, and mixed-signal circuit simulator. It does not own internal domain concepts directly; instead, it delegates to five sub-contexts:

- `netlist-graph` — circuit structure and connectivity
- `device-modeling` — semiconductor device equations and stamps
- `numeric-solver` — MNA matrix assembly, Newton-Raphson, sparse direct solve
- `analysis-orchestration` — DC / AC / transient / noise control loops
- `application-frontend` — PyO3 Python API and CLI

Key invariants: A circuit description entering the system must be parseable into a connected graph. Every analysis request must resolve to a valid control loop with converged solver calls. Results must be immutable and zero-copy across the Python boundary.

## Boundary

- Starts at user input (SPICE netlist, Python builder API, or CLI invocation).
- Ends at rendered results (NumPy arrays, plots, CSV/JSON files, VCD traces).
- Adjacent external systems: digital event simulators (iverilog, Verilator) for mixed-signal cosimulation.

## Ubiquitous Language

- `Circuit` — the top-level object representing a netlist and its associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a requested computation (DC, AC, transient, noise).
- `Netlist` — the textual or programmatic circuit description.
- `Result` — the unified output structure for any analysis.
- `Golden Reference` — a trusted external simulator against which results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a golden reference.

## In-Scope Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/newton-raphson-method]]
- [[concepts/device-modeling]]
- [[concepts/dc-analysis]]
- [[concepts/ac-analysis]]
- [[concepts/transient-analysis]]
- [[concepts/noise-analysis]]
- [[concepts/mixed-level-simulation]]
- [[concepts/golden-reference]]
- [[concepts/event-trace-equivalence]]
- [[concepts/value-change-dump]]
- [[concepts/global-interpreter-lock]]
- [[concepts/ownership]]

## In-Scope Entities

- [[entities/ngspice]]
- [[entities/icarus-verilog]]
- [[entities/sky130-pdk]]
- [[entities/asap7-pdk]]
- [[entities/pyo3]]
- [[entities/russell]]
- [[entities/faer]]

## Relationships

- [[context-maps/circuit-solver]] — Full context map with false-cognate inventory and integration-pattern assignments.
- [[architecture/circuit-solver]] — C4 container diagram showing the six internal containers.
- [[specs/circuit-solver]] — Gherkin acceptance criteria and v1 release scenarios.
- [[vision/circuit-solver]] — Scope, differentiation, and value proposition.
