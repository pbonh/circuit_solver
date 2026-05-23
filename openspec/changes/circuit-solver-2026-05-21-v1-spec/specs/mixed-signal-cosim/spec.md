---
title: "Spec: Mixed-Signal Co-Simulation"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: mixed-signal-cosim
created: 2026-05-21
updated: 2026-05-21
---

# Capability: Mixed-Signal Co-Simulation

Optimistic time synchronization between a continuous-time analog solver
and an external event-driven digital simulator, mediated by a shared
Mixed-Signal Scheduler that issues run-until commands, exchanges
next-event-time information, and performs sparse-checkpoint rollback on
digital misprediction.

## Glossary (inlined from manifest)

- `Circuit` — the top-level object representing a netlist and its
  associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a specific simulation type requested by the user (DC,
  AC, transient, noise).
- `Result` — the unified output structure for any analysis.
- `Waveform` — a time-domain voltage or current signal.
- `Convergence` — success or failure of the overall analysis or
  per-iteration solve.
- `Golden Reference` — a trusted external simulator against which
  results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a
  golden reference.
- `SmallSignal` — the linearized behavior around an operating point.
- `LargeSignal` — the full nonlinear time-domain behavior.

## Personas

- **SimulationEngineer** — an engineer who configures mixed-signal
  simulations; has authority to specify the digital kernel adapter,
  analog/digital boundary signals, and synchronization parameters.
- **ConformanceTester** — an automated agent or engineer who compares
  mixed-signal results against golden references and reports pass/fail.

## Acceptance Criteria

- The Mixed-Signal Scheduler orchestrates the analog solver and an
  external digital simulator (Icarus Verilog or Verilator adapter)
  using optimistic time advancement per ADR-0004.
- The analog solver advances adaptively up to the predicted next
  digital event time, saving a sparse checkpoint at that boundary.
- When the digital simulator reports that no event occurred at the
  predicted time, the Scheduler rolls back the analog state to the
  last checkpoint and resumes with the corrected event time.
- Analog-to-digital and digital-to-analog signal exchange occurs at
  every synchronization point; boundary values are interpolated to the
  event time if the analog step does not land exactly on it.
- The Result contains both analog Waveforms and digital event traces in
  VCD format.
- Event trace equivalence with the Golden Reference is verified at
  every cycle boundary.

## Scenarios

### Scenario: Optimistic advance with correct prediction
```gherkin
Given SimulationEngineer has constructed a mixed-signal Circuit with an analog front-end and a digital Verilog block
And the digital simulator predicts a next event at time 50 ns
When the Scheduler issues a run-until command to the analog solver for 50 ns
And the digital simulator confirms an event at 50 ns
Then the Scheduler commits the analog state at 50 ns
And the Result contains analog Waveforms and digital event traces synchronized at 50 ns
And no rollback occurs
```

### Scenario: Optimistic advance with misprediction requiring rollback
```gherkin
Given SimulationEngineer has constructed a mixed-signal Circuit
And the digital simulator predicts a next event at time 100 ns
When the Scheduler issues a run-until command to the analog solver for 100 ns
And the analog solver saves a sparse checkpoint at 100 ns
And the digital simulator reports no event at 100 ns but an event at 80 ns
Then the Scheduler rolls back the analog state to the checkpoint nearest before 80 ns
And the Scheduler re-issues a run-until command for 80 ns
And the Result contains correct analog Waveforms and digital traces at 80 ns
And the rollback event is recorded in the Result metadata
```

### Scenario: Analog-digital boundary signal exchange
```gherkin
Given SimulationEngineer has configured boundary signals: analog output "vout" driving digital input "din" and digital output "dout" driving analog input "vin"
When the Scheduler reaches a synchronization point at time T
Then the analog solver provides the value of "vout" at time T to the digital simulator as "din"
And the digital simulator provides the value of "dout" at time T to the analog solver as "vin"
And both simulators proceed from time T with the exchanged boundary values
```

### Scenario: Mixed-signal result contains VCD trace
```gherkin
Given SimulationEngineer has completed a mixed-signal simulation with Icarus Verilog as the digital kernel
When the Result is produced
Then the Result contains an analog Waveform section with time-indexed node voltages
And the Result contains a VCD-format digital event trace
And the VCD trace is parseable by standard VCD readers
```

### Scenario: Mixed-signal conformance with event trace equivalence
```gherkin
Given ConformanceTester has a Golden Reference for a mixed-signal simulation including both analog Waveforms and digital event traces
And the tolerance envelope for analog is 1 % relative and for digital is event trace equivalence at cycle boundaries
When ConformanceTester runs the same mixed-signal simulation
Then analog Waveforms match the Golden Reference within the tolerance envelope
And digital event traces are event-trace-equivalent with the Golden Reference at every cycle boundary
And Conformance is reported as "pass"
```

### Scenario: Digital simulator violates next-event-time contract
```gherkin
Given SimulationEngineer has configured a mixed-signal simulation
And the digital simulator reports an event at a time earlier than its previously predicted next-event-time
When the Scheduler detects the contract violation
Then the Scheduler rolls back to the last committed checkpoint before the early event time
And the Scheduler logs a diagnostic warning about the next-event-time contract violation
And the simulation continues from the corrected point
```

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `optimistic-advance-with-correct-prediction`** — task `t_a36ef768` (key `2026-05-21-v1-spec:task-48:ff89c4c7`) merged at `10a0ea1c66972d4b33ea2a75ba4f80cf85188828` by `scientia-integrator`. Verification: `cargo test -p analysis-orchestration` → 259 passed / 0 failed / 1 ignored. Residual risk: Mechanical conflict only (doc-bullet union) resolved by implementer respawn; semantic risk low. Changed files: 2.
- **Scenario `digital-simulator-violates-next-event-time-contract`** — task `t_9181dade` (key `mixed-signal-cosim:ADR-0006:digital-simulator-violates-next-event-time-contract:289b2e9b`) merged at `a7206630b042d706cbde51517deb18ae031a60b1` by `scientia-integrator`. Verification: `cargo fmt/build/clippy/test all green (190 unit + 13 integration tests passed)`. Residual risk: Merge is local-only (origin/main not updated). Prior integrator t_c786184c performed the actual merge. Changed files: 0.
- `t_0e49004b` (#51) — mixed-signal analysis control loop — merged at `2b2b44be4973651f0c98c5895e071dc8786dde4e` via `crates/analysis-orchestration/tests/scenario_mixed_signal_analysis_control_loop_item_51.rs` (test). ADRs: `ADR-0004`, `ADR-0010`. Preflights: fmt OK, clippy OK, test 277 pass/0 fail/1 ignore.
