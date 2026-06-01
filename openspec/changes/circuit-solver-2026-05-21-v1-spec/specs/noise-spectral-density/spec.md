---
title: "Spec: Noise Spectral Density"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: noise-spectral-density
created: 2026-05-21
updated: 2026-05-21
---

# Capability: Noise Spectral Density

AC-variant analysis computing output-referred noise spectral density from
intrinsic device noise sources linearized around a DC OperatingPoint,
producing spectral-density curves versus frequency.

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
- **SimulationEngineer** — an engineer who configures advanced
  simulations including noise analysis; has authority to specify noise
  output nodes and input-referred vs. output-referred modes.
- **ConformanceTester** — an automated agent or engineer who compares
  solver results against golden references and reports pass/fail.

## Acceptance Criteria

- A noise spectral-density Analysis computes output-referred noise
  power spectral density (V²/Hz or A²/Hz) at a specified output node
  over the requested frequency Sweep.
- Each intrinsic device noise source (thermal, shot, flicker) is
  linearized around the OperatingPoint and contributes independently
  to the total output noise.
- The Result contains per-frequency total noise spectral density and,
  optionally, per-device noise contributions.
- Integrated noise over a specified bandwidth is available as a
  summary metric in the Result.
- The noise Analysis uses the same complex-valued faer sparse-direct
  backend as AC small-signal per ADR-0002.
- Results match the Golden Reference within tolerance.

## Scenarios

### Scenario: Noise analysis on a resistive circuit
```gherkin
Given CircuitDesigner has constructed a Circuit containing only resistors and independent sources
And an OperatingPoint has been computed with Convergence status "converged"
When SimulationEngineer submits a noise spectral-density Analysis request for output node "out"
Then the Result contains thermal noise spectral density at every frequency in the Sweep
And the total output noise density at each frequency matches the theoretical 4kTR value within the tolerance envelope
```

### Scenario: Noise analysis with flicker and shot noise contributions
```gherkin
Given CircuitDesigner has constructed a Circuit containing MOSFET devices with flicker noise parameters
And an OperatingPoint has been computed with Convergence status "converged"
When SimulationEngineer submits a noise spectral-density Analysis request
Then the Result contains total output noise spectral density at every frequency
And the Result also contains per-device noise contributions broken down by noise type (thermal, shot, flicker)
And the total noise density matches the Golden Reference within the tolerance envelope
```

### Scenario: Integrated noise over bandwidth
```gherkin
Given CircuitDesigner has constructed a Circuit and obtained noise spectral-density results
And the frequency Sweep spans 1 Hz to 10 MHz
When SimulationEngineer requests integrated noise from 1 kHz to 1 MHz
Then the Result contains the integrated RMS noise voltage over the specified bandwidth
And the integrated noise matches the Golden Reference within the tolerance envelope
```

### Scenario: Noise analysis without prior operating point
```gherkin
Given CircuitDesigner has constructed a Circuit
And no OperatingPoint has been computed for this Circuit
When CircuitDesigner submits a noise spectral-density Analysis request
Then the Simulator first computes a DC OperatingPoint
And the Simulator proceeds with noise linearization at that OperatingPoint
And the Result contains both the OperatingPoint and the noise spectral-density data
```

### Scenario: Noise conformance against ngspice
```gherkin
Given ConformanceTester has a ngspice Golden Reference for noise analysis on a Sky130 PDK test bench
And the tolerance envelope is configured as 2 % relative or 1 nV/√Hz absolute per frequency point
When ConformanceTester runs the noise Analysis on the same Circuit and frequency Sweep
Then every noise spectral-density point matches the Golden Reference within the tolerance envelope
And Conformance is reported as "pass"
```

### Scenario: Noise analysis on circuit with failed operating point
```gherkin
Given CircuitDesigner has constructed a Circuit
And the automatic DC OperatingPoint computation fails with Convergence status "failed"
When CircuitDesigner submits a noise spectral-density Analysis request
Then the Simulator returns a Result with Convergence status "failed"
And the Result contains the DC failure diagnostic
And no noise spectral-density data is produced
```
