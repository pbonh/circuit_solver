---
capability: analog-engine
created: 2026-05-28
---

# Feature: Analog Simulation Engine (DC / AC / Transient / Noise)

Continuous-time analog analyses solved on the pure-Rust hybrid sparse-direct backend (russell for real DC/transient, faer for complex AC) per ADR-0002, validated against the ngspice golden reference on sky130. Steady-state / RF (harmonic balance, shooting) is explicitly OUT of scope for this change (grill oq-steady-state-scope).

## Scenarios

<!-- traces-grill: ha-golden-reference -->
**Scenario: DC operating point matches the golden reference**
```gherkin
Given an analog netlist on the sky130 PDK with a defined DC operating point
And the ngspice golden reference for that operating point
When the analog engine solves the DC operating point
Then every node voltage matches the golden reference within 100 uV or 5%
```

### Scenario: Transient integration matches the golden reference
```gherkin
Given a ring-oscillator transient testbench on sky130
When the engine runs a transient analysis using A-stable backward-Euler / trapezoidal integration
Then the node waveforms match the ngspice golden reference within 5%
```

### Scenario: AC small-signal uses the pure-Rust complex backend
<!-- traces-grill: cc-adr0002-pure-rust -->
```gherkin
Given a small-signal AC analysis request producing a complex (G + jwC) MNA system
When the engine assembles and solves that complex system
Then it is solved by the pure-Rust faer backend with no C/C++ FFI, and gain matches the golden reference within 0.5 dB
```

### Scenario: Non-convergence is guarded, never silently wrong
```gherkin
Given a DC problem that fails plain Newton-Raphson convergence
When the engine applies gmin-stepping then source-stepping continuation
Then it either converges within tolerance or returns a structured non-convergence error (no unconverged result is reported as a solution)
```

### Scenario: The orchestration crate declares an explicit Cargo dependency on the numeric crate
```gherkin
Given the orchestration crate's Cargo.toml
When its [dependencies] are inspected
Then circuit-solver-numeric is present as a direct path dependency and the numeric solver is not accessed via module re-export from any other crate
```
