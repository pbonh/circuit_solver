## ADDED Requirements

### Requirement: Parse SPICE netlist syntax into internal circuit graph with element taxonomy
The system SHALL parse SPICE netlist syntax and construct an internal directed graph representation where node identifiers (strings) are vertices, circuit elements (R, L, C, V, I, VCCS, VCVS, CCCS, CCVS, MOSFET, diode, BJT) are registered with their terminal connectivity and parameters, per [[computer-methods-circuit-analysis-design]] §3 and [[vlsi-graph-methods]].

#### Scenario: Two-port and four-port element parsing
- **WHEN** a netlist contains a resistor `R1 n1 n2 1k`, voltage source `V1 n1 0 5`, VCCS `G1 n3 n4 n1 n2 1m`, and MOSFET `M1 d g s b nch w=10u l=1u`
- **THEN** the parser registers R1 with terminal pair (n1, n2) and value 1k, V1 with terminals (n1, 0) and voltage 5, G1 with control terminals (n1, n2) and output terminals (n3, n4), M1 with all four terminals and aspect ratio W/L=10, and each element is catalogued for subsequent MNA stamping

#### Scenario: Multiport controlled sources and implicit ground node
- **WHEN** the netlist references nodes without explicit node-0 connection and includes a CCVS `H1 out1 out2 in1 in2 10` (transresistance 10 Ω)
- **THEN** the parser identifies the ground node as node 0, creates H1 with input terminals (in1, in2) and output terminals (out1, out2), correctly interprets the transresistance parameter, and validates that all node names are consistent across multiple element references

### Requirement: Stamp parsed elements into sparse Modified Nodal Analysis matrix with conductances, susceptances, and branch variables
The system SHALL construct a sparse MNA conductance matrix G and source vector b by stamping each element according to the Modified Nodal Analysis formulation: resistors contribute conductance stamps to the G matrix, capacitors contribute to the C (susceptance) matrix for transient analysis, voltage sources and inductors append branch-current variables (augmenting G with KVL constraint rows), and controlled sources stamp Jacobian entries for linearization, per [[computer-methods-circuit-analysis-design]] §9 and [[vlsi-graph-methods]].

#### Scenario: Resistor and DC voltage source stamping
- **WHEN** a circuit contains a 1 kΩ resistor between nodes 1 and 2, with a 5 V DC source from node 1 to ground
- **THEN** the MNA stamp adds G = 1e-3 S (conductance of 1/1000) to positions (1,1), (1,2), (2,1), (2,2) with appropriate signs, appends a branch-current row/column for the voltage source, enforces the KVL constraint V1 - V2 = 5 V in the augmented system, and the RHS vector b is populated with the source amplitude

#### Scenario: Inductor branch-current variable extension
- **WHEN** an inductor L is placed between nodes 3 and 4 (for transient analysis)
- **THEN** the MNA augments the system with an additional state variable i_L (inductor current), stamps the relationship V_34 = L × di_L/dt into the discretized system, and reserves the Jacobian entries (∂/∂V, ∂/∂i_L) for the integration method's implicit step solver

#### Scenario: Controlled source Jacobian stamping for Newton-Raphson
- **WHEN** a VCCS with transconductance g_m is stamped into the MNA, where output current depends on control voltage
- **THEN** the MNA matrix includes the linearized Jacobian stamp (∂I_out/∂V_control = g_m), the stamp is updated at each Newton iteration with refreshed partial derivatives from the device model, and the system maintains sparsity by only filling non-zero elements

### Requirement: Detect and report circuit topology errors preventing DC solution
The system SHALL analyze the circuit topology to identify structural defects that make the DC operating point undefined or singular: floating nodes (no DC path to ground through resistances), voltage source loops (series voltage sources with no resistance), and inductor loops (series inductors with no resistance), per [[computer-methods-circuit-analysis-design]] §10.

#### Scenario: Floating node detection with capacitor-only connection
- **WHEN** a node is connected only to capacitor terminals (e.g., one plate of a capacitor, with the other plate unconnected) and current sources, but no resistive path to ground
- **THEN** the topology analyzer reports "Floating node: node_name has no resistive DC path", identifies the capacitor and current source connections, and prevents the MNA matrix construction or marks the matrix as singular with null-space dimension = 1

#### Scenario: Series voltage-source loop detection
- **WHEN** a circuit contains a voltage source loop: V1 from n1 to n2 (voltage V1), V2 from n2 to n1 (voltage V2), with no resistance between them
- **THEN** the topology analyzer detects the cycle in the directed graph of voltage sources, reports "Voltage source loop: [V1, V2] have no resistance; total voltage = V1 + V2, constraint is inconsistent unless V1 + V2 = 0", and prevents MNA construction with exit code indicating singular matrix

#### Scenario: Inductor self-loop (L between same node)
- **WHEN** a netlist contains a line `L1 n1 n1 10u` (inductor connected from node to itself)
- **THEN** the parser detects the zero-voltage inductor (V = 0 but L × di/dt is nonzero), reports "Inductor L1 is a self-loop: no voltage, current undefined", and rejects or warns the user

### Requirement: Verify the MNA conductance matrix is symmetric positive semi-definite (PSD) for purely resistive networks
The system SHALL verify, upon request or in validate mode, that the MNA conductance matrix G is symmetric positive semi-definite for circuits containing only linear resistors, capacitors (in DC analysis, C → ∞ open circuit), and independent sources, per [[computer-methods-circuit-analysis-design]] §9 and [[vlsi-graph-methods]].

#### Scenario: Symmetry check after resistive network stamping
- **WHEN** a resistive-only circuit (no nonlinear elements, no inductors) is stamped into the MNA matrix
- **THEN** the verification routine checks G(i, j) = G(j, i) for all matrix entries to machine precision (relative tolerance ≤ 1e-14 for double precision), reports any asymmetry, and confirms that the matrix is symmetric or flags asymmetry as an implementation bug

#### Scenario: PSD and null-space dimension for floating nodes
- **WHEN** a resistive network has K floating nodes (nodes with no resistive connection to ground) and is stamped into MNA
- **THEN** the eigenvalue check computes eigenvalues of G, verifies that all eigenvalues are ≥ −1e-14 (negative eigenvalues only due to round-off), confirms that exactly K eigenvalues are zero (null-space dimension = K), and reports "Matrix is PSD with null-space rank = K (floating nodes)"

#### Scenario: PSD failure detection for negative-conductance violation
- **WHEN** a user-defined device model (behavioral or empirical) contributes a negative conductance entry (e.g., a resistance value parsed as -100 Ω instead of +100 Ω)
- **THEN** the PSD check detects negative eigenvalues, reports "Matrix is not PSD; negative eigenvalue = -0.01 S at node pair (5, 7)", identifies the likely culprit element, and suggests parameter validation
