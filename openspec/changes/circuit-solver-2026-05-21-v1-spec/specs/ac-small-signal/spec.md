---
title: "Spec: AC Small Signal"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: ac-small-signal
created: 2026-05-21
updated: 2026-05-21
---

# Capability: AC Small Signal

Sinusoidal frequency-domain analysis linearized around a DC OperatingPoint,
reporting magnitude and phase of TransferFunctions versus frequency via
complex-valued sparse-direct LU factorization.

## Glossary (inlined from manifest)

- `Circuit` — the top-level object representing a netlist and its
  associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a specific simulation type requested by the user (DC,
  AC, transient, noise).
- `Result` — the unified output structure for any analysis.
- `OperatingPoint` — the DC steady-state solution used as a reference
  for AC/noise/transient.
- `SmallSignal` — the linearized behavior around an operating point.
- `TransferFunction` — the complex ratio of output to input in AC
  analysis.
- `Sweep` — a sequence of analysis points (voltage, frequency, or
  time).
- `Convergence` — success or failure of the overall analysis or
  per-iteration solve.
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

- An AC small-signal Analysis linearizes the Circuit around a
  previously computed OperatingPoint and solves a complex-valued MNA
  system at each frequency in the Sweep.
- The Result contains TransferFunction data (magnitude in dB and
  phase in degrees) for every requested output/input pair at every
  frequency point.
- If no OperatingPoint has been computed, the Simulator computes one
  automatically before proceeding with AC.
- The complex-valued sparse LU factorization uses the faer backend
  per ADR-0002.
- A frequency Sweep spanning decades produces results matching the
  Golden Reference within tolerance at every point.

## Scenarios

### Scenario: AC analysis with pre-computed operating point
```gherkin
Given CircuitDesigner has constructed a Circuit and obtained an OperatingPoint from a prior DC analysis
And the OperatingPoint Convergence status is "converged"
When CircuitDesigner submits an AC small-signal Analysis request with a frequency Sweep from 1 Hz to 100 MHz
Then the Simulator linearizes the Circuit at the OperatingPoint
And the Result contains magnitude and phase for every output/input pair at every frequency in the Sweep
And every TransferFunction value matches the Golden Reference within the tolerance envelope
```

### Scenario: AC analysis without prior operating point
```gherkin
Given CircuitDesigner has constructed a Circuit
And no OperatingPoint has been computed for this Circuit
When CircuitDesigner submits an AC small-signal Analysis request
Then the Simulator first computes a DC OperatingPoint
And the Simulator proceeds with AC linearization at that OperatingPoint
And the Result contains both the OperatingPoint and the AC frequency-domain data
```

### Scenario: AC analysis on purely linear circuit
```gherkin
Given CircuitDesigner has constructed a Circuit containing only linear elements (R, L, C, independent sources)
When CircuitDesigner submits an AC small-signal Analysis request
Then the Simulator returns a Result with TransferFunction data
And the magnitude response is flat or monotonic as expected by circuit topology
And the Result matches the Golden Reference within the tolerance envelope
```

### Scenario: AC frequency sweep over multiple decades
```gherkin
Given CircuitDesigner has constructed a Circuit with a bandpass filter topology
And the frequency Sweep is logarithmic from 1 kHz to 1 GHz with 100 points per decade
When CircuitDesigner submits an AC small-signal Analysis request
Then the Result contains TransferFunction data at every frequency point
And the bandpass center frequency and Q factor match the Golden Reference within tolerance
And the complex-valued solves use the faer sparse-direct backend
```

### Scenario: AC conformance against ngspice
```gherkin
Given ConformanceTester has a ngspice Golden Reference for an AC analysis on a Sky130 PDK test bench
And the tolerance envelope is configured as 0.1 dB magnitude and 1 degree phase
When ConformanceTester runs the AC small-signal Analysis on the same Circuit
And the same frequency Sweep is used
Then every TransferFunction point matches the Golden Reference within the tolerance envelope
And Conformance is reported as "pass"
```

### Scenario: AC analysis on circuit with failed operating point
```gherkin
Given CircuitDesigner has constructed a Circuit
And the automatic DC OperatingPoint computation fails with Convergence status "failed"
When CircuitDesigner submits an AC small-signal Analysis request
Then the Simulator returns a Result with Convergence status "failed"
And the Result contains the DC failure diagnostic
And no AC frequency-domain data is produced
```

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `scenario-ac-conformance-sky130-ngspice`** — task `t_6c4bab8b` (key `ac-conformance-sky130-ngspice`) merged at `65898aaf71cd6cd2adf6d993739a08dc514b2601` by `scientia-integrator`. Verification: `cargo fmt`, `cargo build`, `cargo clippy`, `cargo test` all pass. Residual risk: none. Changed files: 2.

