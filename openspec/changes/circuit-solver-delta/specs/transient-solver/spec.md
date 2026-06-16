## ADDED Requirements

### Requirement: Integrate circuit differential-algebraic equations (DAEs) using variable-step Radau IIA (5th order) as default integration method with automatic timestep control
The system SHALL integrate the circuit DAE system (arising from the MNA formulation) using variable-step Radau IIA (5th-order, stiffly accurate, L-stable), with automatic internal timestep control driven by local truncation error (LTE) estimates, per [[solving-ode-ii-stiff-dae]] (Hairer & Wanner, the authoritative reference) and [[differential-algebraic-equations]].

#### Scenario: Stiff RC circuit with widely separated time constants
- **WHEN** a transient analysis is initialized on a circuit with τ1 = 1 ns (parasitic capacitance × small resistance) and τ2 = 100 μs (large RC time constant), and the analysis spans 1 ms total
- **THEN** Radau IIA automatically adapts the internal timestep from h ≈ 0.1 ns (to resolve fast transients) to h ≈ 1 μs (when settling slowly), without user intervention, converges to a prescribed LTE tolerance (default 1e-3 V), and completes in ~500–1000 internal steps (vs. ~1 million fixed-step points at Nyquist)

#### Scenario: Mixed-timescale circuit with oscillation and exponential settling
- **WHEN** a circuit contains both fast oscillations (MHz LC tank, 1 ns period) and slow amplitude decay (τ_decay = 10 μs), with stiffness ratio ≈ 1e4
- **THEN** Radau IIA maintains a single variable timestep throughout, uses fine steps (~10 ps) during oscillation to capture the waveform and avoid aliasing, coarser steps (~100 ns) during slow decay, automatically switches between regimes, and achieves convergence without requiring user-specified fixed timestep or multiple nested integrators

### Requirement: Support BDF1 (Backward Euler) and BDF2 (Gear's method) as selectable alternative integration methods with equivalent LTE control and inner NR loop
The system SHALL provide Backward Euler (BDF1, 1st order, A-stable) and Gear's method (BDF2, 2nd order, A(α)-stable, α ≈ 88°) as selectable alternatives to Radau IIA, via a configuration parameter or command-line option, with the same variable timestep control (LTE-based) and per-timestep Newton-Raphson solve, per [[bdf-methods]] and [[stiff-ode-methods]].

#### Scenario: BDF2 fallback selection for backward-compatible SPICE behavior
- **WHEN** a user specifies `integration_method: "BDF2"` in the analysis configuration for a legacy SPICE-compatible mode
- **THEN** the transient integrator switches from Radau IIA to Gear's method, applies the same LTE tolerance logic, automatically computes two-step error estimate (∝ (t_{n} − t_{n-1})^3 for BDF2), and produces results with similar accuracy and (slightly less robust) stability on stiff problems

#### Scenario: BDF1 (Backward Euler) selection for maximum stability
- **WHEN** a user selects `integration_method: "BDF1"` for a weakly nonlinear circuit where A-stability is the paramount concern (avoiding spurious oscillations)
- **THEN** the integrator uses Backward Euler (1st-order), accepts lower accuracy (LTE ~ h^2 vs. h^6 for Radau), and guarantees unconditional stability (no step-size restriction from stability regions), suitable for circuits with sharp nonlinearities (diode switching) where higher-order methods might oscillate

#### Scenario: BDF method fallback on detector timeout
- **WHEN** Radau IIA encounters a detected convergence stall (inner NR loop fails to converge in 50 iterations on a few steps in a row)
- **THEN** the system optionally auto-downgrade to BDF2 for the next N steps (e.g., 10 steps), then retry Radau IIA, as a recovery strategy for pathological transients

### Requirement: Estimate and control Local Truncation Error (LTE) in the voltage domain (node voltages, not charge) using embedded error estimate
The system SHALL estimate the local truncation error at each timestep by comparing two solutions of different orders within the integrator's family (e.g., Radau IIA order 5 vs. order 4 embedded solution, or BDF2 vs. BDF1 error formula), apply the estimate in the voltage domain (||ΔV||∞ or specified relative tolerance on node voltages), and accept or reject the step based on whether LTE < tolerance_V (default 1e-3 V or 0.1% of reference voltage), per [[solving-ode-ii-stiff-dae]] and [[simulation-analog-mixed-signal-circuits]] §LTE-control.

#### Scenario: LTE estimation via embedded order drop
- **WHEN** Radau IIA computes a step using 5th-order solution y_5(t_{n+1}) and an embedded 4th-order solution y_4(t_{n+1})
- **THEN** the local truncation error is estimated as LTE ≈ ||y_5 − y_4||∞ in the voltage state (node voltages, not branch currents), the estimate is normalized by the signal scale (e.g., max(1 V, max_V in the circuit)), and the step is accepted if LTE < 1e-3 V

#### Scenario: LTE tolerance-based step acceptance/rejection
- **WHEN** a transient step produces an LTE estimate of 0.05 V (exceeding the tolerance of 1e-3 V on a 50 V signal swing)
- **THEN** the step is rejected, timestep is halved (h_new = h_old / 2), the same time interval [t_n, t_{n+0.5 h_old}] is re-integrated, and the solution is re-evaluated for LTE acceptance

#### Scenario: Voltage-domain vs. charge-domain control (Spectre approach)
- **WHEN** the integrator controls LTE in the voltage domain rather than charge domain (dQ/dV on capacitors)
- **THEN** the error control directly targets the physical quantity (node voltage) observable in circuit analysis, avoiding spurious refinement at low voltages where charge error is large but voltage error is small, and provides more predictable accuracy (e.g., 1e-3 V is 1e-3 V regardless of capacitor value)

### Requirement: Halve timestep when LTE exceeds tolerance; grow timestep (up to h_max) when LTE is well below tolerance (adaptive timestep growth)
The system SHALL implement adaptive timestep control: when LTE > tolerance, reject the step and halve h; when LTE < tolerance/10 (well below tolerance), accept the step and increase h by a factor k (default k = 1.5) up to a user-specified maximum h_max (or heuristically set to shortest-period/10), per [[solving-ode-ii-stiff-dae]].

#### Scenario: Timestep halving on LTE violation
- **WHEN** a transient step at h = 10 ns produces an estimated LTE = 2e-3 V (exceeding tolerance = 1e-3 V)
- **THEN** the step is rejected, h is reduced to 5 ns, the time window [t_n, t_n + 5ns] is re-solved, and if LTE ≤ 1e-3 V the step is accepted; if not, h is halved again to 2.5 ns

#### Scenario: Timestep growth acceleration on tight tolerance
- **WHEN** a transient step at h = 1 ns produces LTE = 1e-5 V (well below tolerance = 1e-3 V, ratio 100×)
- **THEN** the step is accepted and h is increased by factor 1.5 to h_new = 1.5 ns for the next step, allowing the integrator to skip over quiet regions and accelerate long-duration settling

#### Scenario: h_max enforcement and signal period detection
- **WHEN** timestep growth would produce h > h_max (e.g., h_max set to 100 ns to resolve a 1 μs ripple period), or h_max is auto-set heuristically based on the fastest signal (T_min / 10)
- **THEN** the system caps h at h_max, preventing overshooting of fast-changing features; if a signal has components faster than h_max, LTE will exceed tolerance and steps will be refined automatically

#### Scenario: Minimum timestep floor and stiffness adaptation
- **WHEN** repeated LTE-based halving would reduce h below a machine-precision floor (e.g., h_min = 1 ps, set to prevent loss of significance in arithmetic)
- **THEN** the system halts and reports "Minimum timestep h_min = 1 ps reached, but LTE still > tolerance; circuit integration is ill-conditioned" and allows user to adjust tolerance or circuit parameters

### Requirement: Run Newton-Raphson inner loop at each accepted timestep to solve the implicit discretized algebraic system with Gmin insertion and KCL convergence check
The system SHALL, at each accepted timestep t_{n+1}, form the discretized implicit algebraic system (arising from the integration formula: e.g., Radau IIA implicit formula evaluates the RHS at internal stages, requiring nonlinear solves), and solve it via Newton-Raphson with the same Gmin insertion, KCL residue-norm convergence check (||residual||∞ < 1e-6 A), and Gmin/source stepping homotopy as the DC solver, per [[differential-algebraic-equations]] and [[computer-methods-circuit-analysis-design]] §13.

#### Scenario: Inner NR loop convergence for smooth transient step
- **WHEN** a transient step at h = 10 ns is accepted and the implicit Radau IIA stage equations are formed
- **THEN** the Newton-Raphson loop is invoked on each Radau stage (3 implicit stages), Jacobian is computed including both the circuit's Jacobian and the integration formula's discretization terms (df/dx terms), and each stage converges in 3–5 iterations to ||residual||∞ < 1e-6 A using the prior stage solution as initial guess

#### Scenario: Inner NR failure triggers step rejection
- **WHEN** a Newton-Raphson solve within a transient step fails (residue norm stalls, oscillates, or exceeds 1e-3 A after 100 iterations)
- **THEN** the entire transient timestep is rejected, h is halved, and the time interval [t_n, t_n + h/2] is re-attempted with refined timestep

#### Scenario: Stage initialization and Jacobian reuse
- **WHEN** Radau IIA computes stage 1, 2, 3 at a single timestep, each requiring an implicit NR solve
- **THEN** the stage 1 NR solution is used as initial guess for stage 2 (with adjustment), and stage 2 for stage 3, exploiting the fact that stages are temporally close; Jacobian can be reused (frozen) across stages for efficiency if spectral radius is acceptable, or refreshed at each stage for robustness

### Requirement: Detect and flag integration failures (repeated step rejections, inability to reduce LTE below tolerance) and report the failing timepoint with diagnostic information
The system SHALL monitor the timestep history during transient integration and detect integration failure: when h undergoes ≥ 5 consecutive halvings at approximately the same timepoint (e.g., trying to integrate past t = 50 μs but h → h/2 → h/4 → ... → h/32 without LTE convergence), classify this as a singularity or ill-posedness in the DAE, and report the failure with timestamp, minimum achieved timestep, and suggestions for remediation, per [[solving-ode-ii-stiff-dae]] (Hairer's analysis of step-rejectionpatterns).

#### Scenario: Repeated step rejection detection at stiff singularity
- **WHEN** a transient integration encounters a point (e.g., t = 45.2 μs) where device switching or nonlinearity becomes nearly singular, and the integrator attempts h = 10ns → 5ns → 2.5ns → 1.25ns → 0.625ns → 0.3125ns without achieving LTE < tolerance
- **THEN** the system detects 5 consecutive halvings, reports "Integration failure at t = 45.2 μs: unable to achieve LTE < 1e-3 V; minimum h = 0.3125 ns. Likely cause: device singularity (e.g., MOSFET threshold crossing with capacitive load), negative differential resistance, or model discontinuity. Recommendation: check device models for C1/C2 smoothness, examine netlist around t = 45.2 μs."

#### Scenario: Minimum timestep floor reached
- **WHEN** h shrinks to a minimum floor h_min = 1 ps (set to prevent loss-of-significance in 64-bit floating point)
- **THEN** the system reports "Integration stalled at t = 45.2 μs, h = h_min = 1 ps, LTE = 5e-3 V >> tolerance = 1e-3 V. DAE is ill-posed at this point. Options: (1) coarsen LTE tolerance, (2) refine circuit model, (3) add explicit damping/series resistance to reduce stiffness."

#### Scenario: User-specified max-rejection limit
- **WHEN** a user sets max_rejections = 10 (stop after 10 failed steps at same timepoint)
- **THEN** the integrator tracks rejections, halts gracefully when the limit is reached, and reports the state (node voltages, currents) at the last accepted time and diagnostics for the failure

#### Scenario: Divergence of Newton-Raphson inner loop during transient
- **WHEN** a transient step's inner Newton-Raphson loop diverges (residue increases monotonically), indicating that the implicit stage equation is singular or ill-scaled
- **THEN** the system immediately rejects that stage's computation, triggers step rejection, and reduces h; divergence is flagged as a red indicator of a fundamental model or scaling issue
