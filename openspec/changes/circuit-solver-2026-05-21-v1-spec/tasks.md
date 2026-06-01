---
title: "Tasks: Deliver circuit-solver v1"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
manifest_tasks: development/manifests/circuit-solver/2026-05-21-v1-spec/tasks.md
created: 2025-07-18
---

# Implementation Plan

## Workspace & Shared Infrastructure

- [ ] **1.** Create Cargo workspace with five crate stubs (`netlist-graph`, `device-modeling`, `numeric-solver`, `analysis-orchestration`, `application-frontend`) plus `circuit-solver-py` PyO3 extension crate — non-behavioral
- [ ] **2.** Define shared types crate (`circuit-solver-types`) with `NodeId`, `BranchId`, `ElementId`, `ModelName`, `ConvergenceStatus`, `AnalysisType` enums/structs — @adr: ADR-0006 (depends on #1)
- [ ] **3.** Implement `FlattenedStructure` struct in `numeric-solver` with incidence mapping, node-to-branch map, element enumeration, and ground-reference bookkeeping — @spec: dc-operating-point#linear-resistive-dc-operating-point (depends on #2)
- [ ] **4.** Implement topology checker in `netlist-graph` that traverses flattened incidence and classifies nodes as grounded / possibly-grounded / floating, producing `TopologyReport` — @adr: ADR-0009 (depends on #3)

## Capability: dc-operating-point

- [ ] **5.** Implement `CircuitGraph` builder in `netlist-graph`: add element, add wire, add model, add subcircuit, expand subcircuits, build() returns immutable `CircuitGraph` — @spec: python-frontend#incremental-circuit-construction-via-builder-api (depends on #2)
- [ ] **6.** Implement Pass 1 structure flattening: read `CircuitGraph` once, produce `FlattenedStructure` with full incidence including ground node — @spec: dc-operating-point#linear-resistive-dc-operating-point (depends on #5)
- [ ] **7.** Implement closed-enum `DeviceModel` with `Diode(DiodeParams)`, `BJT(BJTParams)`, `MOSFET(MOSFETParams)` variants; each variant owns its `ModelParameters` inline — @adr: ADR-0005 (depends on #2)
- [ ] **8.** Implement `LinearizedModel` stamp generation and Jacobian computation via `match` on `DeviceModel` enum — @adr: ADR-0005 (depends on #7)
- [ ] **9.** Implement Diode stamp (Shockley equation, companion model for NR) — @spec: dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence (depends on #8)
- [ ] **10.** Implement BJT stamp (Ebers-Moll / Gummel-Poon) — @spec: dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence (depends on #8)
- [ ] **11.** Implement MOSFET Level-1 stamp — @spec: dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence (depends on #8)
- [ ] **12.** Implement MOSFET BSIM3v3 stamp — @spec: dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence (depends on #8)
- [ ] **13.** Implement MOSFET BSIM4 stamp — @spec: dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence (depends on #8)
- [ ] **14.** Implement Pass 2 MNA matrix assembly: stamp linearized models into full MNA matrix (including ground row/column) from `FlattenedStructure` — @spec: dc-operating-point#linear-resistive-dc-operating-point (depends on #6, #8)
- [ ] **15.** Implement sub-view extractor: ground-suppressed unknowns, constraint masks (source-stepping, Gmin-stepping) — @adr: ADR-0003 (depends on #14)
- [ ] **16.** Integrate `russell_sparse` real-valued sparse-direct LU dispatch behind `LinearSolver` trait — @adr: ADR-0002 (depends on #14)
- [ ] **17.** Implement `NewtonRaphsonDriver` with dual convergence criterion (update norm + residue norm) — @adr: ADR-0006 (depends on #16)
- [ ] **18.** Implement Gmin-stepping homotopy: add shunt conductances to ground, gradually reduce, solve at each step — @spec: dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy (depends on #17)
- [ ] **19.** Implement source-stepping homotopy as alternative convergence aid — @spec: dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy (depends on #17)
- [ ] **20.** Implement DC analysis control loop: accept `AnalysisRequest`, drive NR driver, return `OperatingPoint` with `ConvergenceStatus` — @spec: dc-operating-point#linear-resistive-dc-operating-point (depends on #17)
- [ ] **21.** Implement DC Sweep: iterate over source parameter range, produce one `OperatingPoint` per sweep point, result addressable by index — @spec: dc-operating-point#dc-sweep-over-a-voltage-source (depends on #20)
- [ ] **22.** Implement DC convergence failure path: return last-iterate voltages + diagnostic when NR and homotopy both fail — @spec: dc-operating-point#dc-operating-point-convergence-failure (depends on #20)

## Capability: ac-small-signal

- [ ] **23.** Integrate `faer` complex-valued sparse-direct LU dispatch behind `LinearSolver` trait — @adr: ADR-0002 (depends on #14)
- [ ] **24.** Implement AC sub-view extraction: complex-valued MNA augmentation (G + jωC) around operating point — @spec: ac-small-signal#ac-analysis-with-pre-computed-operating-point (depends on #23)
- [ ] **25.** Implement AC analysis control loop: linearize at operating point, solve complex system at each frequency, compute TransferFunction (magnitude dB + phase degrees) — @spec: ac-small-signal#ac-analysis-with-pre-computed-operating-point @spec: ac-small-signal#ac-analysis-on-purely-linear-circuit (depends on #24)
- [ ] **26.** Implement auto-DC computation: when no `OperatingPoint` cached, run DC first, then proceed with AC; return both in Result — @spec: ac-small-signal#ac-analysis-without-prior-operating-point (depends on #25, #20)
- [ ] **27.** Implement AC failure short-circuit: if auto-DC fails, return Result with Convergence "failed" and DC diagnostic, no frequency data — @spec: ac-small-signal#ac-analysis-on-circuit-with-failed-operating-point (depends on #26)
- [ ] **28.** Implement logarithmic frequency Sweep with configurable points-per-decade — @spec: ac-small-signal#ac-frequency-sweep-over-multiple-decades (depends on #25)

## Capability: transient-time-domain

- [ ] **29.** Implement Backward Euler companion models for reactive elements (capacitor, inductor) — @spec: transient-time-domain#transient-analysis-with-default-integration-method (depends on #8)
- [ ] **30.** Implement Trapezoidal companion models for reactive elements — @spec: transient-time-domain#transient-analysis-with-trapezoidal-integration (depends on #8)
- [ ] **31.** Implement Gear-2 BDF companion models for reactive elements — @spec: transient-time-domain#transient-analysis-with-gear-2-bdf-integration (depends on #8)
- [ ] **32.** Implement adaptive timestepping with LTE estimator: compute local truncation error, reject step if above tolerance, reduce h and re-solve — @spec: transient-time-domain#adaptive-timestepping-rejects-and-re-solves (depends on #29)
- [ ] **33.** Implement transient analysis control loop: compute initial DC operating point (or accept UIC), step through time interval with selected integration method, return Waveforms — @spec: transient-time-domain#transient-analysis-with-default-integration-method (depends on #32, #20)
- [ ] **34.** Implement UIC (Use Initial Conditions) mode: skip DC operating point, start from user-supplied node voltages — @spec: transient-time-domain#transient-analysis-with-uic-initial-conditions (depends on #33)
- [x] **35.** Implement timestep history metadata in Result — @spec: transient-time-domain#adaptive-timestepping-rejects-and-re-solves (depends on #32)

## Capability: noise-spectral-density

- [ ] **36.** Implement intrinsic device noise source modeling: thermal (4kTG), shot (2qI), flicker (KF/I^AF / f) contributions per device — @spec: noise-spectral-density#noise-analysis-on-a-resistive-circuit (depends on #8)
- [ ] **37.** Implement noise analysis control loop: linearize at operating point, build noise transfer matrices at each frequency, compute output-referred spectral density — @spec: noise-spectral-density#noise-analysis-on-a-resistive-circuit @spec: noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point (depends on #36, #25)
- [ ] **38.** Implement per-device noise breakdown (optional): attach per-device, per-noise-type contribution to Result — @spec: noise-spectral-density#noise-analysis-with-flicker-and-shot-noise-contributions (depends on #37)
- [ ] **39.** Implement integrated noise over bandwidth: trapezoidal integration of spectral density over user-specified band, return RMS noise voltage — @spec: noise-spectral-density#integrated-noise-over-bandwidth (depends on #37)
- [ ] **40.** Implement auto-DC for noise analysis (same pattern as AC) — @spec: noise-spectral-density#noise-analysis-without-prior-operating-point (depends on #37, #20)
- [ ] **41.** Implement noise failure short-circuit (same pattern as AC) — @spec: noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point (depends on #40)

## Capability: mixed-signal-cosim

- [ ] **42.** Implement `MixedSignalScheduler` core: own both kernel handles, issue run-until to analog, query next-event-time from digital — @adr: ADR-0004 (depends on #33)
- [ ] **43.** Implement sparse checkpoint manager: save node voltages + reactive companion state at predicted event boundaries — @adr: ADR-0004 (depends on #42)
- [ ] **44.** Implement rollback handler: restore analog state from checkpoint, re-issue run-until with corrected time — @spec: mixed-signal-cosim#optimistic-advance-with-misprediction-requiring-rollback (depends on #43)
- [ ] **45.** Implement boundary signal exchanger with zero-order hold default: hold last analog value constant until event time — @spec: mixed-signal-cosim#analog-digital-boundary-signal-exchange @adr: ADR-0007 (depends on #42)
- [ ] **46.** Implement linear interpolation opt-in at boundary: retain two most recent solution vectors, interpolate at event time — @adr: ADR-0007 (depends on #45)
- [ ] **47.** Implement Icarus Verilog adapter: next-event-time query, event delivery, rollback-to-checkpoint protocol via VVP runtime — @spec: mixed-signal-cosim#optimistic-advance-with-correct-prediction (depends on #42)
- [ ] **48.** Implement Verilator adapter: same interface as Icarus adapter, different runtime binding — @spec: mixed-signal-cosim#optimistic-advance-with-correct-prediction (depends on #42)
- [ ] **49.** Implement digital contract violation detection: detect event earlier than predicted next-event-time, rollback + log diagnostic warning — @spec: mixed-signal-cosim#digital-simulator-violates-next-event-time-contract (depends on #44)
- [ ] **50.** Implement VCD trace output in Result — @spec: mixed-signal-cosim#mixed-signal-result-contains-vcd-trace (depends on #42)
- [ ] **51.** Implement mixed-signal analysis control loop: orchestrate scheduler, collect analog Waveforms + digital VCD traces, produce unified Result — @spec: mixed-signal-cosim#optimistic-advance-with-correct-prediction (depends on #42, #50)

## Capability: python-frontend

- [ ] **52.** Implement PyO3 `CircuitBuilder` class: add_element, add_wire, add_model, add_subcircuit methods delegating to Rust — @spec: python-frontend#incremental-circuit-construction-via-builder-api (depends on #5)
- [ ] **53.** Implement `CircuitBuilder.build()` returning immutable `CircuitGraph` PyO3 handle — @spec: python-frontend#incremental-circuit-construction-via-builder-api (depends on #52)
- [ ] **54.** Implement `ImmutableHandleError` on attempted mutation of built `CircuitGraph` — @spec: python-frontend#immutable-circuit-graph-prevents-post-build-mutation (depends on #53)
- [ ] **55.** Implement builder isolation: multiple builds from same builder produce independent graphs — @spec: python-frontend#builder-isolation-across-multiple-builds (depends on #52)
- [ ] **56.** Implement `AnalysisRequest` Python class: analysis type, sweep parameters, integration method, boundary interpolation option — @spec: python-frontend#analysis-request-and-result-retrieval (depends on #53)
- [ ] **57.** Implement `Result` Python class: node voltages, branch currents, Waveforms, TransferFunctions accessible by name — @spec: python-frontend#analysis-request-and-result-retrieval (depends on #56)
- [ ] **58.** Implement zero-copy NumPy result arrays: PyO3 numpy feature, Rust-owned memory viewed as ndarray dtype float64 — @spec: python-frontend#zero-copy-numpy-result-arrays (depends on #57)
- [ ] **59.** Implement GIL release around every solver entry point via `Python::allow_threads` — @spec: python-frontend#gil-release-during-simulation (depends on #57)
- [ ] **60.** Implement SPICE netlist file parsing: `circuit_solver.parse_netlist(path)` returning `CircuitGraph` — @spec: python-frontend#spice-netlist-file-parsing (depends on #53)
- [ ] **61.** Implement `NetlistParseError` with line number and unrecognized token — @spec: python-frontend#error-on-malformed-netlist (depends on #60)

## Cross-Cutting: Conformance Harness

- [ ] **62.** Implement conformance harness framework: load ngspice golden reference files, compare per node using max(relative, absolute) tolerance — @adr: ADR-0008 (depends on #57)
- [ ] **63.** Implement DC conformance test: Sky130 PDK test bench, 1 % relative / 1 mV absolute — @spec: dc-operating-point#conformance-test-against-ngspice-golden-reference (depends on #62, #20)
- [ ] **64.** Implement AC conformance test: Sky130 PDK, 0.1 dB magnitude / 1° phase — @spec: ac-small-signal#ac-conformance-against-ngspice (depends on #62, #25)
- [ ] **65.** Implement transient conformance test: Sky130 PDK, 1 % relative / 1 mV absolute per time point per node — @spec: transient-time-domain#transient-conformance-against-ngspice (depends on #62, #33)
- [ ] **66.** Implement noise conformance test: Sky130 PDK, 2 % relative / 1 nV/√Hz absolute — @spec: noise-spectral-density#noise-conformance-against-ngspice (depends on #62, #37)
- [ ] **67.** Implement mixed-signal conformance test: analog tolerance + digital event-trace equivalence at cycle boundaries — @spec: mixed-signal-cosim#mixed-signal-conformance-with-event-trace-equivalence (depends on #62, #51)
- [ ] **68.** Implement ASAP7 PDK conformance test variant for DC and transient — @spec: dc-operating-point#conformance-test-against-ngspice-golden-reference (depends on #63, #65)

## Cross-Cutting: Documentation & CI

- [ ] **69.** Write crate-level rustdoc for all five workspace crates — non-behavioral (depends on #20, #25, #33, #37, #51)
- [ ] **70.** Write Python API reference (docstrings + Sphinx or mkdocs) — non-behavioral (depends on #61)
- [ ] **71.** Set up CI pipeline: cargo test + cargo clippy + maturin develop + conformance harness — non-behavioral (depends on #62)
- [ ] **72.** Configure maturin build for wheel production — non-behavioral (depends on #61)
