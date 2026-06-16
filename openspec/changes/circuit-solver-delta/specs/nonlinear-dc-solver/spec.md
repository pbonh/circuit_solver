## ADDED Requirements

### Requirement: Solve nonlinear MNA DC equation G(x)·x = b(x) via Newton-Raphson iteration with initial guess strategy
The system SHALL implement Newton-Raphson iteration to solve the nonlinear algebraic system G(x)·x = b(x) arising from the MNA formulation with nonlinear elements, starting from an initial guess (typically zero node voltages or from a prior analysis), computing the Jacobian J = ∂(G·x − b)/∂x at each iteration, and updating x_new = x_old − J^{−1}·residual until convergence, per [[newton-raphson]] and [[computer-methods-circuit-analysis-design]] §12.

#### Scenario: Convergence on linear resistive circuit (one iteration)
- **WHEN** a circuit contains only linear resistors, voltage sources, and current sources (no nonlinear elements), and NR is initialized from zero voltages
- **THEN** the Jacobian is constant and equals the MNA conductance matrix G, the first iteration computes the analytical solution, the residual ||G·x − b||∞ becomes < 1e-6 A, and iteration terminates in exactly one step

#### Scenario: Convergence on nonlinear circuit with smooth devices
- **WHEN** a circuit contains a diode or MOSFET biased in the forward/conducting region, with transconductance ≥ 1e-6 S, and initial guess is a flat 0 V
- **THEN** the Jacobian is well-conditioned (condition number ≲ 1e6), NR iterates with superlinear convergence rate, and ||residual|| drops below 1e-6 A within ≤ 10 iterations for strongly nonlinear elements (diode turn-on) and ≤ 5 iterations for weakly nonlinear (MOSFET in saturation)

### Requirement: Use Kirchhoff's Current Law (KCL) residue norm as primary convergence criterion, not ΔV or ΔI
The system SHALL compute the residual vector r = G(x)·x − b(x) at each Newton iteration and evaluate the convergence criterion ||r||∞ < tolerance_I (default 1e-6 A), where tolerance_I is specified in Amperes (not Volts or dimensionless), and SHALL NOT use ΔV (voltage step) or ΔI (current step) as the primary criterion, per [[simulation-analog-mixed-signal-circuits]] §Newton-Raphson and [[computer-methods-circuit-analysis-design]] §12.

#### Scenario: KCL residue enforces global charge balance
- **WHEN** Newton-Raphson iteration is performed on a circuit with multiple nodes and nonlinear devices, and the residual r = G·x − b is computed at each step
- **THEN** the residual vector represents the imbalance in Kirchhoff's current law (outgoing currents minus incoming currents at each node, in Amperes), ||r||∞ is the maximum KCL violation across all nodes, and convergence when ||r||∞ < 1e-6 A guarantees that current conservation is satisfied to within 1 μA at every node

#### Scenario: Tolerance-based stopping prevents false convergence
- **WHEN** the KCL residue norm ||r||∞ drops below 1e-6 A (the tolerance)
- **THEN** Newton-Raphson immediately halts and reports convergence, even if the voltage change ΔV in the last iteration was large (e.g., ΔV > 0.1 V at some node), because the physical constraint (current balance) is satisfied

#### Scenario: ΔV-based criterion avoids false positives on weak conductances
- **WHEN** a circuit contains a very high-impedance path (e.g., reversed-biased diode with G ≈ 1e-12 S) where small voltage changes cause tiny current changes
- **THEN** monitoring ΔV instead of residual could halt prematurely (ΔV < 1e-3 V but ||r|| > 1e-3 A), whereas the KCL-based criterion correctly requires current balance, not just small voltage steps, catching the error

### Requirement: Insert Gmin conductance (default 1e-12 S) across nonlinear device terminal pairs to prevent floating-node singularities
The system SHALL automatically add a small parallel conductance Gmin (default value 1e-12 S, configurable) between each terminal pair of nonlinear devices (diodes, MOSFETs, BJTs) during MNA construction, to ensure the Jacobian remains non-singular even when a device operates in an off state (reverse-biased, subthreshold, or cutoff), per [[simulation-analog-mixed-signal-circuits]] §Gmin-damping and [[computer-methods-circuit-analysis-design]] §12.

#### Scenario: Gmin insertion for reverse-biased diode
- **WHEN** a diode is reverse-biased (V_anode < V_cathode) with magnitude ≥ 100 mV, giving device conductance g_d ≈ 1e-15 S (exponentially suppressed)
- **THEN** the MNA includes a parallel Gmin = 1e-12 S between anode and cathode, the effective conductance becomes g_eff ≈ 1e-12 S, the Jacobian diagonal entry remains ≥ 1e-12 (avoiding ill-conditioning), and the Newton-Raphson Jacobian determinant does not approach zero

#### Scenario: Gmin insertion for MOSFET in strong subthreshold
- **WHEN** a MOSFET is biased below threshold (V_gs < V_t by 100 mV), with drain-source conductance g_ds ≈ 1e-15 S (nanoamp leakage)
- **THEN** the MNA stamps Gmin between all terminal pairs (drain-source, gate-source, bulk-source), the effective conductance is dominated by Gmin, the device remains observable by Newton-Raphson (Jacobian rank = 4 for nch), and convergence is robust even when the device is "off"

#### Scenario: Gmin conductance path through substrate
- **WHEN** a bulk-tied circuit has multiple MOSFETs sharing the bulk (substrate), and some are OFF
- **THEN** Gmin provides a conducting path through the substrate for all nodes, preventing any node from becoming electrically isolated during Newton-Raphson iteration, ensuring global connectivity of the conductance graph

### Requirement: Attempt Gmin stepping homotopy when Newton-Raphson convergence fails within maximum iterations
The system SHALL detect Newton-Raphson convergence failure (residual not decreasing monotonically, or iteration count exceeding max_iter, typically 100) and automatically invoke Gmin stepping homotopy: gradually reduce Gmin from a large initial value (e.g., 1e-3 S) toward the nominal value (1e-12 S) over a sequence of DC operating-point solves, using the converged solution from each step as the initial guess for the next step, per [[homotopy-methods]] and [[simulation-analog-mixed-signal-circuits]] §homotopy.

#### Scenario: Gmin stepping recovery on strong nonlinearity
- **WHEN** a diode or MOSFET circuit exhibits S-curve / multi-valued behavior (e.g., negative differential resistance), and bare Newton-Raphson fails to converge after 100 iterations
- **THEN** Gmin stepping is invoked: Gmin = 1e-3 S (load-line damping), solve DC problem (converges quickly), reduce Gmin to 1e-4 S using prior solution as initial guess, repeat until Gmin = 1e-12 S, at which point the original circuit is solved and convergence is achieved

#### Scenario: Gmin stepping path tracing
- **WHEN** Gmin stepping is in progress at Gmin = 1e-4 S and a solve converges
- **THEN** the system stores the converged solution, extracts initial guess for Gmin = 1e-5 S, and passes it to the next solve; this path-following reduces the required NR iterations per step and improves robustness by avoiding sharp turns in solution space

#### Scenario: Gmin stepping max iterations and failure fallback
- **WHEN** Gmin stepping completes its sequence (e.g., 10 steps from 1e-3 to 1e-12 S) but convergence is still not achieved at the final Gmin value
- **THEN** the system falls through to source stepping as the secondary recovery method (see next requirement), or reports failure if source stepping is not enabled

### Requirement: Attempt source stepping homotopy as secondary recovery when Gmin stepping is exhausted
The system SHALL, if Gmin stepping does not recover convergence, invoke source stepping homotopy as a backup: scale all independent voltage and current sources uniformly from zero to nominal amplitude over a sequence of DC solves (e.g., 0% → 1% → 10% → 30% → 100%), using the prior solution as initial guess for each step, per [[homotopy-methods]] and [[simulation-analog-mixed-signal-circuits]] §source-stepping.

#### Scenario: Source stepping on bistable circuit
- **WHEN** a bistable/latch circuit (cross-coupled NAND gates or flip-flop) is initialized and bare Newton-Raphson plus Gmin stepping both fail to find a stable operating point
- **THEN** source stepping is invoked: scale all voltage/current sources to 1% of nominal, solve DC (converges to a weakly excited operating point), scale to 10%, solve using prior solution, continue to 100%, and the system gradually ramps into the circuit's equilibrium state without jumping between bistable points

#### Scenario: Source stepping with multiple independent sources
- **WHEN** a circuit has multiple independent sources (e.g., V1 = 5 V, I1 = 100 mA) at different scales and source stepping is active
- **THEN** both sources are scaled by the same factor λ ∈ [0, 1] at each homotopy step: V_stepped = λ × 5 V, I_stepped = λ × 100 mA, maintaining the circuit's load-line structure while smoothly transitioning from low to nominal drive

#### Scenario: Source stepping convergence at reduced bias
- **WHEN** source stepping reduces source magnitudes to 10% and the DC problem converges
- **THEN** the converged solution is stored, source magnitudes are increased to 30%, the DC solver is called again with the prior solution as initial guess, and the sequence continues toward 100%

### Requirement: Report convergence failure explicitly with iteration count, last residue norm, and diagnostic circuit state when all recovery methods are exhausted
The system SHALL, when Newton-Raphson, Gmin stepping, and source stepping all fail to converge, report the failure with: the iteration count (or homotopy step count if applicable), the last computed residue norm ||r||∞ in Amperes, the rate of convergence analysis (linear, superlinear, stalled, or diverging), and the circuit state (node voltages, branch currents, device operating points) at the failure point, logged to a diagnostic file for post-mortem analysis, per [[computer-methods-circuit-analysis-design]] §13 (circuit design constraints for convergence).

#### Scenario: Failure report after Gmin + source stepping exhaustion
- **WHEN** both Gmin stepping (10 steps) and source stepping (100 steps toward full source amplitude) complete without convergence, and the final residue norm is still > 1e-6 A
- **THEN** the system reports: "DC convergence failed: final residue norm = 3.2e-4 A after 10 Gmin steps + 100 source steps; last iter=100 in final solve; convergence rate: linear (slope 0.8). Circuit state logged to <filename>. Recommended: check device parameter smoothness, verify source polarity, examine netlist topology."

#### Scenario: Divergence detection mid-iteration
- **WHEN** a single Newton-Raphson step computes a residue norm that is larger than the prior iteration (divergence), and this occurs before max_iter is reached
- **THEN** the solver logs the divergence, reports "Iteration N: residue = X A (diverging, prior = Y A), Jacobian ill-conditioned or device singularity detected", and can optionally trigger automatic source reduction (partial source stepping) as an emergency fallback

#### Scenario: Minimum timestep reached in transient DC fallback
- **WHEN** DC analysis invokes a pseudo-transient continuation (damped transient path) as an extreme fallback, and the pseudo-time step shrinks to a minimum h_min = 1 ps without convergence
- **THEN** the system reports "Pseudo-transient continuation stalled at h_min = 1 ps, residue norm = 2.1e-3 A. Circuit is likely ill-posed (negative resistance, parameter error, or unphysical load line)." and halts with diagnostic file pointer
