---
title: "Spec: DC Operating Point"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: dc-operating-point
created: 2026-05-21
updated: 2026-05-21
---

# Capability: DC Operating Point

Steady-state equilibrium computation via Newton-Raphson with homotopy
aids, producing node voltages and branch currents that satisfy KCL/KVL
with zero time derivatives.

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
- `Convergence` — success or failure of the overall analysis or
  per-iteration solve.
- `Golden Reference` — a trusted external simulator against which
  results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a
  golden reference.
- `Sweep` — a sequence of analysis points (voltage, frequency, or
  time).

## Personas

- **CircuitDesigner** — an engineer who constructs circuits and runs
  analyses to verify electrical behavior; has authority to submit
  analysis requests and read results.
- **ConformanceTester** — an automated agent or engineer who compares
  solver results against golden references and reports pass/fail; has
  authority to read results and tolerance configurations.

## Acceptance Criteria

- A DC operating-point analysis on a linear resistive circuit produces
  node voltages and branch currents matching the golden reference within
  the configured tolerance envelope.
- A DC operating-point analysis on a nonlinear circuit (containing
  diodes, BJTs, or MOSFETs) converges via Newton-Raphson iteration and
  produces an OperatingPoint matching the golden reference within
  tolerance.
- When Newton-Raphson fails to converge directly, the simulator
  applies source-stepping or Gmin-stepping homotopy and reports
  Convergence status.
- The OperatingPoint result is immutable once produced; subsequent
  analyses may reference it but cannot mutate it.
- A DC Sweep over a source parameter produces a Result containing one
  OperatingPoint per sweep point.

## Scenarios

### Scenario: Linear resistive DC operating point
```gherkin
Given CircuitDesigner has constructed a Circuit from a linear resistive netlist
And the Circuit contains no nonlinear devices
When CircuitDesigner submits a DC operating-point Analysis request
Then the Simulator returns a Result containing an OperatingPoint
And every node voltage and branch current in the OperatingPoint matches the Golden Reference within the tolerance envelope
And the Convergence status is "converged"
```

### Scenario: Nonlinear DC operating point with direct convergence
```gherkin
Given CircuitDesigner has constructed a Circuit from a netlist containing MOSFET devices
And the MOSFET devices use closed-enum DeviceModel dispatch
When CircuitDesigner submits a DC operating-point Analysis request
Then the Simulator returns a Result containing an OperatingPoint
And every node voltage matches the Golden Reference within the tolerance envelope
And the Convergence status is "converged"
And the Newton-Raphson iteration count is reported in the Result
```

### Scenario: DC operating point with Gmin-stepping homotopy
```gherkin
Given CircuitDesigner has constructed a Circuit from a netlist containing floating nodes
And direct Newton-Raphson on the Circuit fails to converge
When CircuitDesigner submits a DC operating-point Analysis request
Then the Simulator applies Gmin-stepping homotopy
And the Simulator returns a Result containing an OperatingPoint
And the Convergence status is "converged-via-homotopy"
And the homotopy step count is reported in the Result
```

### Scenario: DC operating point convergence failure
```gherkin
Given CircuitDesigner has constructed a Circuit with no DC path to ground on node "n5"
And neither direct Newton-Raphson nor homotopy methods converge
When CircuitDesigner submits a DC operating-point Analysis request
Then the Simulator returns a Result with Convergence status "failed"
And the Result contains the last-iterate node voltages and a diagnostic message
And no OperatingPoint is produced
```

### Scenario: DC Sweep over a voltage source
```gherkin
Given CircuitDesigner has constructed a Circuit with a swept voltage source "V1"
And the sweep range is 0 V to 5 V in 11 steps
When CircuitDesigner submits a DC Sweep Analysis request
Then the Simulator returns a Result containing 11 OperatingPoints
And each OperatingPoint matches the corresponding Golden Reference within the tolerance envelope
And the Result is addressable by sweep index
```

### Scenario: Conformance test against ngspice golden reference
```gherkin
Given ConformanceTester has a ngspice Golden Reference for a Sky130 PDK test bench
And the tolerance envelope is configured as 1 % relative or 1 mV absolute per node
When ConformanceTester runs the DC operating-point Analysis on the same Circuit
Then every node voltage in the Result matches the Golden Reference within the tolerance envelope
And Conformance is reported as "pass"
```

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `conformance-test-against-ngspice-golden-reference` (ASAP7 variant)** — task `t_04b4e126` (key `t_04b4e126`) merged at `55100e5a3c51e2570e780d9623e2133af5ea8561` by `scientia-integrator`. Verification: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (190 passed, 0 failed), `cargo doc --workspace --no-deps` all clean. Residual risk: None. Changed files: 3.
