---
title: "Threshold-Crossing Event Scheduler as the Mixed-Signal Bridge"
status: proposed
date: 2026-06-16
decision-makers:
  - circuit-solver-team
consulted: []
informed: []
---

# Threshold-Crossing Event Scheduler as the Mixed-Signal Bridge

## Context and Problem Statement

The Circuit Solver Delta must couple continuous-time analog simulation (Newton-Raphson nonlinear solve on MNA matrices over transient time steps) with discrete-time digital logic simulation (Verilog-AMS behavioral models operating on clocked or event-driven semantics). The core challenge is synchronizing these two temporal domains: when does an analog voltage crossing a logic threshold trigger a digital event, and when does a digital state change require recomputation of the analog circuit?

Without a well-defined bridge, timing violations (race conditions), missed logic events, and spurious analog restarts degrade both correctness and simulation performance.

## Decision Drivers

1. **Correctness of event causality**: Thresholds must be detected precisely; causality between analog crossings and digital transitions must be preserved (no events "lost" between analog solver steps).
2. **Simulation performance**: The number of analog solver restarts when a crossing is detected impacts total wall-clock time; unnecessary restarts waste computation.
3. **Verilog-AMS compatibility**: The simulator must ingest Verilog-AMS behavioral models cleanly, particularly `transition()` and `slew()` operators for injecting digital state changes back into the analog domain.
4. **Formal verification readiness**: The event dispatch mechanism should be amenable to formal analysis or model checking to ensure no hidden timing bugs.

## Considered Options

### Option 1: Full DEVS Formalism (Atomic/Coupled Model Hierarchy)
Implement the Discrete Event System Specification (DEVS) with atomic models for analog subcircuits and digital components, coupled via a DEVS coordinator. Each model publishes its next event time and processes inputs at those times.

**Pros:**
- Maximally formal and mathematically rigorous.
- Provably correct event ordering and causality.
- Extensible to complex hierarchies (e.g., hierarchical VLSI blocks).

**Cons:**
- Significant engineering overhead for a first release; DEVS coordinators require careful synchronization logic.
- The analog solver (Newton-Raphson + transient integration) is inherently iterative and does not fit the "atomic model with fixed event times" assumption.
- Not necessary for monolithic SPICE-style simulation where all components are tightly coupled.

### Option 2: Threshold-Crossing Event Scheduler (Simpler, SPICE-Standard)
Monitor analog solver output for voltage/current crossings of pre-defined thresholds. When a crossing is detected, inject a discrete event (digital state transition) and either roll back the analog solver or accept the crossing and proceed to the next time step with updated initial conditions.

**Pros:**
- Standard in SPICE/Spectre; decades of proven practice.
- Minimal formalism overhead; events are detected reactively as analog computation unfolds.
- Natural integration with Verilog-AMS `transition()` and `slew()` operators for bidirectional coupling.
- Straightforward to verify by comparing edge timings to reference simulators.

**Cons:**
- Less formally provable than DEVS; relies on threshold detection heuristics (bisection, event detection ODE).
- If event ordering becomes critical, may require backtracking or event reordering logic that complicates the implementation.

### Option 3: Co-Simulation via FMI (Functional Mockup Interface)
Export the analog solver as one FMU (Functional Mockup Unit) and digital components as another; orchestrate via an FMI master controller.

**Pros:**
- Separation of concerns; analog and digital simulators can be independently maintained.
- Standard toolchain; commercial simulators already export FMU models.

**Cons:**
- Overkill for a monolithic codebase where all components are in-memory.
- Adds inter-process or network overhead; slower than tight integration.
- Requires an external FMI master; not self-contained.

### Option 4: Separate Analog and Digital Runs with Waveform Exchange
Run analog and digital simulation separately, exchange waveforms (VCD-style) between runs, and iterate until convergence.

**Pros:**
- Completely decoupled; teams can develop analog and digital simulators independently.

**Cons:**
- Slow for iterative refinement and debugging.
- Difficult to capture transient causality; missed events across run boundaries.
- Not suitable for interactive or exploratory simulation workflows.

## Decision Outcome

**Chosen option: Threshold-crossing event scheduler with Verilog-AMS `transition()`/`slew()` injection.**

This approach balances simplicity, proven industry practice, and integration with modern hardware description languages. Threshold crossing is the standard mechanism in SPICE and Spectre; DEVS formalism adds overhead not justified in a first release; FMI is unnecessarily heavyweight for a monolithic codebase; and separate runs are too slow for iterative simulation.

Verilog-AMS `transition()` and `slew()` operators natively cover the digital→analog coupling path, allowing Verilog-AMS behavioral models to inject state changes and slew rates directly into the analog circuit without a separate transformation layer.

## Consequences

- **Less formally provable than DEVS**: Event causality is validated empirically (comparison to SPICE reference outputs) rather than by proof. Future changes to threshold detection or event ordering logic should include regression tests.
- **Potential event ordering complexity**: If event causality becomes critical for verification (e.g., proving absence of race conditions), the implementation may need to revisit event detection, including backtracking or event reordering logic.
- **Analog solver responsibilities**: The analog solver must reliably detect threshold crossings. Two strategies:
  - **Bisection rollback**: After each transient integration step, check if any node crossed a threshold; if so, bisect the time step and recompute until the crossing is localized.
  - **Event detection ODE**: Augment the transient integrator with an event detection ODE extension (standard in Runge-Kutta and BDF methods) to refine the crossing time without bisection.
- **Verilog-AMS interoperability required**: Full parsing and execution of Verilog-AMS `transition()`, `slew()`, and event-driven logic is a prerequisite; incomplete support will limit model compatibility.

## Confirmation

Verification strategy:

1. **Functional correctness**: Simulate a CMOS inverter chain (3–5 stages) with threshold crossings at 100 mV hysteresis. Output a digital state trace (time, node, digital value) and compare edge timings to SPICE (ngspice or Spectre) reference within 1 ps tolerance.
2. **No missed events**: Run the same inverter chain under slew-rate constraints (e.g., 1 V/ns input slew); confirm that all threshold crossings are detected and no digital states are skipped.
3. **Analog solver efficiency**: Measure the number of analog solver restarts (bisections or event refines) per simulation; a well-tuned implementation should require ≤ 2 restarts per crossing event.
4. **Regression suite**: Include CMOS logic, RTL behavioral models with Verilog-AMS `transition()`, and mixed-signal test cases in the continuous integration suite.

## Pros and Cons of the Options

| Aspect | DEVS | Threshold Crossing | FMI | Separate Runs |
|--------|------|-------------------|-----|----------------|
| **Formalism** | Highest | Moderate | Medium | Lowest |
| **Implementation complexity** | Very high | Low | High | Medium |
| **Performance** | Moderate | High | Low (inter-process) | Very low |
| **Industry precedent** | Academic; not in SPICE | SPICE/Spectre standard | Emerging; growing adoption | Rare; research only |
| **Verilog-AMS integration** | Requires mapping | Native with `transition()`/`slew()` | Via FMU wrapping | Post-hoc exchange |
| **Extensibility** | Excellent for hierarchy | Good for monolithic code | Good for multi-tool flow | Poor; requires re-convergence |
| **Verification** | Formal proof possible | Empirical comparison to reference | Formal proof at FMU boundary | Difficult; iterative |

---

## Evidence

This decision is grounded in the following wiki evidence:
- [[verilog-ams]] — Verilog-AMS analog extensions and `transition()`/`slew()` operators.
- [[simulation-analog-mixed-signal-circuits]] — Mixed-signal simulation architectures in commercial tools.
- [[devs-simulation]] — DEVS formalism, atomic models, and coupled hierarchies.
- [[modeling-simulation-systems]] — Foundational simulation semantics and causality models.
