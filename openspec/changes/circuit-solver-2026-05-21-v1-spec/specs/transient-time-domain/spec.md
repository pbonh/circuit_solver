---
title: "Spec: Transient Time Domain"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: transient-time-domain
created: 2026-05-21
updated: 2026-05-21
---

# Capability: Transient Time Domain

Nonlinear time-domain simulation with adaptive timestepping and implicit
integration methods (Backward Euler, Trapezoidal, Gear BDF), producing
Waveforms as time-indexed vectors of node voltages and branch currents.

## Glossary (inlined from manifest)

- `Circuit` — the top-level object representing a netlist and its
  associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a specific simulation type requested by the user (DC,
  AC, transient, noise).
- `Netlist` — the textual or programmatic circuit description.
- `Result` — the unified output structure for any analysis.
- `OperatingPoint` — the DC steady-state solution used as a reference
  for AC/noise/transient.
- `Waveform` — a time-domain voltage or current signal.
- `LargeSignal` — the full nonlinear time-domain behavior.
- `Sweep` — a sequence of analysis points (voltage, frequency, or
  time).
- `Convergence` — success or failure of the overall analysis or
  per-iteration solve.
- `UIC` — Use Initial Conditions, bypassing the DC operating-point
  calculation.
- `Golden Reference` — a trusted external simulator against which
  results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a
  golden reference.

## Personas

- **CircuitDesigner** — an engineer who constructs circuits and runs
  analyses to verify electrical behavior; has authority to submit
  analysis requests and read results.
- **ConformanceTester** — an automated agent or engineer who compares
  solver results against golden references and reports pass/fail.

## Acceptance Criteria

- A transient Analysis computes Waveforms for all observed node voltages
  and branch currents over the requested time interval.
- The Simulator supports at least three implicit integration methods:
  Backward Euler, Trapezoidal, and Gear-2 BDF, selectable per analysis
  request.
- Adaptive timestepping adjusts the step size based on local truncation
  error estimation; rejected steps are re-solved at a smaller step.
- The Simulator uses the russell real-valued sparse-direct backend per
  ADR-0002 for each transient solve.
- When UIC is specified, the initial OperatingPoint calculation is
  skipped and user-supplied initial conditions are used instead.
- The Result matches the Golden Reference Waveforms within the
  tolerance envelope at every reported time point.

## Scenarios

### Scenario: Transient analysis with default integration method
```gherkin
Given CircuitDesigner has constructed a Circuit with a pulsed voltage source
And the transient time interval is 0 s to 100 ns
When CircuitDesigner submits a transient Analysis request
Then the Simulator computes a DC OperatingPoint as the initial state
And the Simulator returns a Result containing Waveforms for all observed nodes
And every Waveform matches the Golden Reference within the tolerance envelope at every time point
```

### Scenario: Transient analysis with Trapezoidal integration
```gherkin
Given CircuitDesigner has constructed a Circuit with an RLC tank
And the integration method is set to Trapezoidal
When CircuitDesigner submits a transient Analysis request
Then the Simulator uses Trapezoidal integration at each timestep
And the Result contains Waveforms with no artificial numerical damping beyond the tolerance envelope
```

### Scenario: Transient analysis with Gear-2 BDF integration
```gherkin
Given CircuitDesigner has constructed a Circuit with stiff device dynamics
And the integration method is set to Gear-2 BDF
When CircuitDesigner submits a transient Analysis request
Then the Simulator uses Gear-2 BDF integration
And the Result contains Waveforms that remain stable throughout the simulation interval
And the Waveforms match the Golden Reference within the tolerance envelope
```

### Scenario: Adaptive timestepping rejects and re-solves
```gherkin
Given CircuitDesigner has constructed a Circuit with rapidly switching inputs
And the initial timestep is set to 1 ns
When the Simulator estimates a local truncation error exceeding the error tolerance
Then the Simulator rejects the current step
And the Simulator re-solves at a smaller timestep
And the final Result contains only accepted time points
And the timestep history is available in the Result metadata
```

### Scenario: Transient analysis with UIC initial conditions
```gherkin
Given CircuitDesigner has constructed a Circuit
And CircuitDesigner specifies UIC with initial node voltages for node "n1" = 3.3 V
When CircuitDesigner submits a transient Analysis request with UIC flag
Then the Simulator skips the DC OperatingPoint computation
And the Simulator starts the transient solve using the user-supplied initial conditions
And the Waveform at node "n1" begins at 3.3 V at time 0 s
```

### Scenario: Transient conformance against ngspice
```gherkin
Given ConformanceTester has a ngspice Golden Reference for a transient analysis on a Sky130 PDK test bench
And the tolerance envelope is configured as 1 % relative or 1 mV absolute per time point per node
When ConformanceTester runs the transient Analysis on the same Circuit with the same time interval and method
Then every Waveform matches the Golden Reference within the tolerance envelope at every reported time point
And Conformance is reported as "pass"
```
