# Tasks — Circuit Solver Delta

<!-- Ordered, independently verifiable tasks grouped by container (C4 architecture.md).
     Each task is one PR-sized unit of work. -->

## mna-formulation

- [ ] 1. Define `CircuitGraph` IR and `NodeId` type; implement ground-node seeding
- [ ] 2. Implement SPICE netlist tokenizer (R, L, C, V, I, VCCS, VCVS, CCCS, CCVS element lines)
- [ ] 3. Extend parser to handle MOSFET, diode, and BJT instance lines with parameter extraction
- [ ] 4. Implement Verilog-AMS `.vams` subset parser: `module`, `analog` block, `idt`/`ddt`/`transition`/`slew`
- [ ] 5. Implement `MnaMatrix` sparse data structure (CSR/COO) with `stamp(row, col, val)` API
- [ ] 6. Implement stamping for linear elements: resistor, capacitor (transient stamp), inductor branch-current extension, independent V/I sources
- [ ] 7. Implement stamping for controlled sources: VCCS, VCVS, CCCS, CCVS Jacobian stubs
- [ ] 8. Implement topology validator: floating-node detection (BFS from ground), voltage-source-loop detection (DFS on V-L subgraph), inductor-loop detection
- [ ] 9. Implement `VarMap` — maps node names and branch-current variables to MNA row/column indices
- [ ] 10. Write verification tests: resistor divider MNA matches hand-computed; voltage-source loop triggers `TopologyError`; floating-node detected and reported

## device-models

- [ ] 11. Define `DeviceModel` trait with `terminals`, `stamp_linear`, `stamp_nonlinear`, `is_smooth`
- [ ] 12. Implement `Resistor`, `Capacitor`, `Inductor`, `VoltageSource`, `CurrentSource` as `DeviceModel`
- [ ] 13. Implement `Diode` model: Shockley I-V with tangent-line clamping at 40·V_T, junction capacitance, series resistance
- [ ] 14. Implement `MosfetLevel1` model: SPICE Level 1 square-law I-V, Meyer gate capacitance
- [ ] 15. Implement `BjtEbersMoll` model: Ebers-Moll forward/reverse regions, base-emitter and base-collector junctions
- [ ] 16. Implement `VerilogAmsBlock` behavioral model: evaluate `analog` block, stamp result as VCCS/CCCS; support `idt`/`ddt` via trapezoidal approximation in the model
- [ ] 17. Write device model tests: diode I-V at forward bias matches Shockley within 1%; MOSFET saturation region I_D matches Level 1 formula; C2 smoothness check passes for all models

## nonlinear-dc-solver

- [ ] 18. Implement `SparseLU`: Markowitz pivot selection, fill-in minimization, factorize + solve API
- [ ] 19. Implement `NewtonRaphson`: assemble Jacobian from device `stamp_nonlinear`, call `SparseLU`, apply KCL residue check (`||G·x - b||∞ < i_tol` AND `||Δx||∞ < v_tol`)
- [ ] 20. Implement Gmin insertion: add Gmin (default 1e-12 S) to all nonlinear device terminal diagonal entries before NR
- [ ] 21. Implement `HomotopyEngine::gmin_stepping`: ramp Gmin from 1e-3 S → 1e-12 S in log steps, using each step's solution as next initial guess
- [ ] 22. Implement `HomotopyEngine::source_stepping`: ramp V/I sources from 0 → full value in linear steps
- [ ] 23. Implement `DcAnalysis::run`: orchestrate NR → Gmin stepping → source stepping; return `DcSolution` or `ConvergenceError`
- [ ] 24. Write DC analysis tests: simple resistor divider converges in ≤3 iterations; diode+resistor DC solution matches hand-computed; Gmin stepping recovers a poorly-conditioned circuit; non-convergence reported with residue norm

## transient-solver

- [ ] 25. Implement `RadauIIA` integrator: 3-stage (order 5) Radau IIA butcher tableau; stage system assembly; embedded error estimate for LTE
- [ ] 26. Implement `Bdf` integrator: BDF1 and BDF2 coefficient tables; history buffer management; LTE estimate via Richardson extrapolation
- [ ] 27. Implement adaptive step controller: accept/reject logic on LTE vs tolerance; halve on violation; grow by 1.5× when LTE < tol/10; cap at h_max; floor at h_min; report `IntegrationError` after 5 consecutive failures
- [ ] 28. Implement `TransientAnalysis::run`: loop over timesteps, sample waveforms at accepted steps, return `TransientSolution { times, waveforms }`
- [ ] 29. Write transient tests: stiff RC ladder (`τ_fast = 1 ns`, `τ_slow = 1 μs`) with Radau IIA converges to analytic solution within 0.1%; BDF2 selection via config gives consistent results; integration failure reported at correct timepoint

## mixed-signal

- [ ] 30. Implement `ThresholdDetector`: monitor `x[node]` at each accepted transient step; bisect within step when crossing `v_high`/`v_low`; emit `DigitalEvent { time, node, state }`
- [ ] 31. Implement `EventScheduler`: min-heap of `DigitalEvent`; `next_event_time()` API; consumed by `TransientAnalysis` to cap `h` before digital boundary
- [ ] 32. Implement `WaveformInjector`: given a sequence of `(time, voltage)` digital events and `tr`/`tf` parameters, stamp a time-varying piecewise-linear voltage source into the MNA matrix at each timestep
- [ ] 33. Implement Verilog-AMS `transition`/`slew` operator semantics in `VerilogAmsBlock` behavioral model
- [ ] 34. Write mixed-signal tests: CMOS inverter threshold crossing detected within 1 ps of reference SPICE; digital event aligns analog timestep grid; digital PWM source drives RC filter transient correctly

## analysis-output

- [ ] 35. Implement `AcAnalysis::run`: linearize at DC operating point, sweep frequencies, solve `(G_0 + jωC)·V = b_ac` via complex `SparseLU`, output `AcSolution`
- [ ] 36. Implement `FourierAnalysis`: DFT of sampled transient waveform at uniformly resampled points (cubic spline interpolation); output magnitude and phase spectra
- [ ] 37. Implement `NutmegWriter`: write Nutmeg binary format header and waveform blocks compatible with ngscope/WaveView
- [ ] 38. Implement `VcdWriter`: write IEEE 1364 VCD for digital state traces from `EventScheduler`
- [ ] 39. Implement `ParquetWriter`: write columnar Parquet file with time index column and per-node voltage columns
- [ ] 40. Implement `PyResult` via PyO3: expose `SimResult.time()`, `SimResult.voltage(node: str)`, `SimResult.current(element: str)` as `numpy.ndarray`
- [ ] 41. Write output tests: Nutmeg file parsed by reference reader matches source waveform within float precision; Parquet columns match node names in netlist; PyResult voltage array shape equals `(n_timepoints,)`

## Integration and CLI

- [ ] 42. Implement `circuit_solver_delta` CLI: `solve <netlist> --analysis dc|ac|tran --output <path> --format nutmeg|vcd|parquet`
- [ ] 43. Implement end-to-end integration test: simulate standard RC circuit (`R=1kΩ`, `C=1nF`, pulse input), verify transient waveform peak and settling time against analytic solution within 1%
- [ ] 44. Implement end-to-end mixed-signal test: CMOS inverter chain (3 stages, behavioral MOSFET), verify output waveform digital transitions within 10 ps of reference
