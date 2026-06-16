## ADDED Requirements

### Requirement: Diode model with Shockley equation and junction capacitance
The system SHALL implement a smooth (C2-continuous) diode model comprising the Shockley exponential I-V characteristic, junction (depletion-region) capacitance, and series resistance, as grounded in [[computer-methods-circuit-analysis-design]] §11 and [[pn-junction]].

#### Scenario: Diode forward bias I-V characteristic
- **WHEN** a diode is DC-biased in the forward-conducting region (V_d ≈ 0.6–0.8 V for silicon at 25°C)
- **THEN** the Newton-Raphson solver computes node voltage and branch current with smooth derivatives, converging in <5 iterations at accuracy criterion 1e-6 V, and the I-V curve matches Shockley: I = I_s × (exp(V/nV_t) − 1) within numerical error tolerance

#### Scenario: Diode reverse bias and junction capacitance
- **WHEN** the diode is reverse-biased (V_d < 0) with moderate reverse voltage (−5 V to 0 V)
- **THEN** the junction capacitance is computed per the depletion-capacitance model, the I-V curve is smooth with no numerical kinks, and the effective capacitance (dQ/dV) is continuous across V_d = 0 (forward-to-reverse transition)

### Requirement: MOSFET Level 1 model with square-law I-V and Meyer capacitance
The system SHALL implement a SPICE Level 1 MOSFET model with square-law drain current (cutoff/triode/saturation regions), Meyer (polysilicon-gate) capacitance model, and body effect, per [[computer-methods-circuit-analysis-design]] §11 and [[mosfet-physics]].

#### Scenario: MOSFET saturation region transconductance
- **WHEN** the MOSFET is biased in the saturation region (V_gs > V_t, V_ds ≥ V_gs − V_t)
- **THEN** the drain current follows the square-law I_d = (W/L)/2 × μ_n × C_ox × (V_gs − V_t)² with smooth derivatives across the saturation boundary, and the effective transconductance g_m = ∂I_d/∂V_gs is continuous (no discontinuity at saturation boundary)

#### Scenario: MOSFET gate capacitance in switching transient
- **WHEN** a transient analysis exercises the MOSFET gate node with a rising slew-rate input (e.g., 1 V/ns)
- **THEN** the Meyer capacitance distributes between gate-drain, gate-source, and gate-bulk according to the inversion-layer charge density, the total charge is conserved (Kirchhoff current law), and the capacitance model exhibits C1 continuity through the triode-saturation transition

### Requirement: BJT model with Ebers-Moll forward and reverse regions
The system SHALL implement a SPICE Ebers-Moll BJT model covering forward-active, reverse-active, saturation, and cutoff regions with smooth (C2) I-V and capacitance models, per [[computer-methods-circuit-analysis-design]] §11 and [[pn-junction]].

#### Scenario: BJT active region gain and saturation
- **WHEN** the BJT is DC-biased in the forward-active region (V_be ≈ 0.6–0.7 V, V_bc < 0.4 V)
- **THEN** the collector current follows the Ebers-Moll relation I_c = β × I_b (or via exponential forms) with smooth derivatives, the Early effect models the V_ce-dependent finite output resistance (r_o), and the model transitions smoothly to saturation when V_bc rises above approximately 0.4 V

#### Scenario: BJT reverse-mode (reverse-active) operation
- **WHEN** the BJT is biased with V_be < 0 and V_bc > 0.6 V (collector-base junction forward-biased, base-emitter junction reverse-biased)
- **THEN** the model switches to reverse-active mode with the roles of collector and emitter swapped, the reverse β is computed from the reverse saturation current ratio, and the transconductance and capacitances are continuous across the forward-to-reverse mode boundary

### Requirement: Passive element stamps for R, L, C, mutual inductance, and independent sources
The system SHALL implement Modified Nodal Analysis (MNA) stamps for passive linear elements (resistor, capacitor, inductor, mutual inductance) and independent voltage/current sources, per [[computer-methods-circuit-analysis-design]] §9 and [[vlsi-graph-methods]].

#### Scenario: Resistor, capacitor, inductor linear stamps
- **WHEN** the circuit netlist includes R, C, and L elements connected between pairs of nodes
- **THEN** the MNA matrix is augmented with the appropriate stamps: (G matrix for R; C matrix for capacitor; augmented branch-current variable for L), the resulting G·x = b system has the circuit Laplacian structure, and solving yields correct node voltages and branch currents

#### Scenario: Coupled inductor mutual inductance and transformer
- **WHEN** two inductors are coupled with a coupling coefficient k (e.g., k = 0.99 for a transformer)
- **THEN** the MNA matrix includes the mutual inductance coupling term (L_m = k × √(L1 × L2)), the system correctly models the secondary induced voltage (V_2 = M × dI_1/dt), and the magnetic energy is conserved per the transformer power balance

#### Scenario: Independent voltage and current source stamps
- **WHEN** the netlist includes independent DC voltage sources (VSDC), AC voltage sources, and independent current sources (ISDC, ISAC)
- **THEN** voltage sources augment the MNA with an additional branch-current variable and a KVL constraint row, current sources are stamped directly as current injection at nodes, DC analysis solves for the operating point, and AC analysis computes small-signal transfer functions relative to the AC source amplitude

### Requirement: Device parameter validation and rejection of non-smooth (C0) models
The system SHALL validate all device model parameters at netlist parse time and reject parameter sets that produce non-smooth (i.e., discontinuous or C0-only) model functions, logging the offending parameter name and reason, per [[computer-methods-circuit-analysis-design]] §12 (Newton-Raphson convergence requirement).

#### Scenario: Diode parameter validation
- **WHEN** the netlist specifies a diode with parameters (Is, n, Rs) and the saturation current Is is zero or negative
- **THEN** the parser logs an error "Diode parameter Is=<value>: must be positive for Shockley smoothness", rejects the netlist, and halts with exit code 1

#### Scenario: MOSFET parameter smoothness check
- **WHEN** the netlist specifies a MOSFET with parameter kp (transconductance parameter) = 0 or negative, or with threshold voltage V_to that would produce a discontinuous I_d(V_gs) curve
- **THEN** the parser logs an error "MOSFET parameter <name>=<value>: produces non-smooth I-V characteristic", rejects the model instantiation, and prevents the circuit from loading

#### Scenario: Capacitance non-linearity C0 detection
- **WHEN** a device model (e.g., voltage-dependent capacitor) specifies a piecewise-constant capacitance (C0 function) or a capacitance function with a discontinuous derivative
- **THEN** the model loader detects the discontinuity, logs a warning or error with the parameter that causes it, and either rejects the model or forces a behavioral event-driven discontinuity handler at the identified threshold
