---
title: "Modified Nodal Analysis (MNA) as the canonical circuit formulation"
status: proposed
date: 2026-06-16
decision-makers:
  - circuit-solver-team
consulted: []
informed: []
---
# Modified Nodal Analysis (MNA) as the canonical circuit formulation


## Context and Problem Statement

Circuit Solver Delta must choose a mathematical formulation for representing circuits before DC analysis, AC analysis, and transient simulation. The formulation defines:
- How circuit elements (R, L, C, V, I, transistors) contribute to the system of equations
- Whether voltage sources and inductors can be handled natively or require special treatment
- The sparsity structure of the system matrix
- Compatibility with SPICE netlists
- Potential for future symbolic analysis (BDD/DDD, sensitivity analysis)

The formulation choice affects:
- Ability to preserve SPICE compatibility (netlists, device models, transient semantics)
- Size and sparsity of the resulting matrix (which governs solver speed)
- Handling of mutual inductance, voltage sources, and current-controlled elements
- Future extensibility to symbolic analysis and graph-based circuit optimization

## Decision Drivers

1. **SPICE compatibility**: Modern circuit simulators (Spectre, HSpice, NGSPICE) are built on MNA. Compatibility enables migration of existing designs and enables comparison with reference implementations.
2. **Handling of voltage sources and inductors**: Nodal analysis alone cannot natively represent voltage sources (which specify node voltage directly rather than element current). MNA extends nodal with branch current variables to handle this.
3. **Sparsity preservation**: The MNA matrix is the circuit Laplacian; its sparsity directly reflects circuit topology. This maximizes the efficiency of sparse matrix solvers.
4. **Mutual inductance and coupled elements**: Multi-winding transformers and coupled inductors require the MNA framework to represent coupling through shared branch currents.
5. **Future symbolic analysis**: Control-theory techniques (Bode plots, sensitivity), reliability analysis, and behavioral optimization all target MNA matrices or equivalent Laplacian representations.
6. **Mathematical clarity**: MNA is a well-studied, peer-reviewed formulation with extensive academic literature; no ambiguity in semantics.

## Considered Options

### Option 1: Modified Nodal Analysis (MNA)
- **Pros**:
  - Standard in all modern simulators (SPICE, Spectre, HSpice, NGSPICE); universal compatibility.
  - Extends pure nodal analysis with branch current variables; natively handles voltage sources, inductors, and current-controlled elements.
  - Preserves sparsity; each element contributes to a small set of matrix entries matching its topology.
  - Straightforward stamp algorithm; each element type has a well-defined stamp (Vladimirescu *Computer Methods for Circuit Analysis and Design*).
  - The MNA matrix is the circuit Laplacian; graph-theoretic analysis (spanning trees, node elimination) applies directly.
  - Symbolic analysis (sensitivity, behavioral optimization) targets MNA matrices or equivalent Laplacian representations.
  - Index-1 DAE structure natural for transient integration (Radau IIA handles index-1 DAEs natively).
- **Cons**:
  - Matrix size includes branch current variables; slightly larger than pure nodal (e.g., n nodes + m voltage sources).
  - Require care with singular components (floating voltage sources without current sink).
  - Less intuitive for newcomers; state-space representation more familiar to control theorists.

### Option 2: Pure Nodal Analysis (without branch currents)
- **Pros**:
  - Smallest matrix size; only node voltages as variables.
  - Simple stamp algorithm; each resistor is a direct conductance stamp.
  - Intuitive for students; common in textbooks.
- **Cons**:
  - **Cannot handle voltage sources natively**; must be decomposed into current sources (requiring source transformation or Thevenin equivalent).
  - Inductors cannot be represented directly; require transformation to augmented node set.
  - Current-controlled elements (VCCS, CCVS) require branch current variables anyway; pure nodal provides no net benefit.
  - No advantage over MNA; forces awkward workarounds.

### Option 3: State-Space Representation (x-formulation)
- **Pros**:
  - Familiar to control theorists; state variables are physical (node voltages, inductor currents).
  - Suitable for control-law derivation and Bode plot analysis.
  - Compact for linear circuits; state vector size is number of reactive elements.
- **Cons**:
  - **Loses SPICE compatibility**; state variables are not directly mapped from netlist.
  - Non-linear device models (diodes, transistors) less natural in state-space form; still require implicit equations.
  - Dense matrix operations on implicit derivatives; loses sparsity.
  - Deriving state-space representation from netlist requires symbolic manipulation (not easily automated).
  - Mutual inductance and coupled elements require explicit graph transformation.

### Option 4: Tableau Method
- **Pros**:
  - Complete representation; all voltages and currents explicitly included.
  - No hidden variables or constraints; fully general.
  - Trivial to extend to new element types.
- **Cons**:
  - Matrix is larger (2n + m variables for n nodes, m edges).
  - Loses sparsity benefits; tableau is denser than MNA.
  - No standard stamp algorithm; requires case-by-case derivation.
  - Solver performance degraded due to larger, denser system.

## Decision Outcome

**Decision**: Adopt **Modified Nodal Analysis (MNA)** as the canonical formulation for Circuit Solver Delta.

**Rationale**:
1. **Universal standard**: All modern industrial simulators (SPICE, Spectre, HSpice, NGSPICE, commercial EDA tools) use MNA. This is not a matter of opinion; it is the de facto standard.
2. **Elegant extension of nodal analysis**: MNA adds branch current variables only where necessary (voltage sources, inductors, current-controlled elements), preserving sparsity while enabling native handling of these constructs.
3. **The MNA matrix is the circuit Laplacian**: Graph theory directly applies. Node elimination, spanning-tree algorithms, and symmetry analysis all operate on the MNA structure. This opens future extensions to symbolic analysis and optimization.
4. **Index-1 DAE structure**: MNA naturally produces index-1 differential-algebraic equations; Radau IIA and other modern integrators handle this without artificial index reduction.
5. **Sparsity preservation**: The sparsity pattern of the MNA matrix directly reflects circuit topology. A 1000-node circuit has a sparse matrix; algorithms exploit this (sparse LU, iterative solvers).
6. **Device model compatibility**: The MNA framework is the lingua franca for semiconductor device models (BSIM, MOS Level, diode models); all published models assume MNA stamps.

## Consequences

1. **Matrix size**: MNA matrix is n×n where n = nodes + branch currents. For a circuit with k voltage sources and inductors, the matrix is approximately (n_nodes + k) × (n_nodes + k). Pure nodal would be n_nodes × n_nodes, but pure nodal cannot handle k voltage sources; MNA is a necessary small increase.
2. **State-space export**: Control theorists may want state-space representation for analysis. This can be derived post-hoc from MNA via standard linear-system techniques (Gramian-based model reduction, state-space realization); not a consequence, but a design point.
3. **Singular components**: A floating voltage source (not forming a loop with a current sink) is singular in MNA. Mitigation: circuit validation layer rejects such topologies or adds a parallel conductance.
4. **Symbolic analysis**: The MNA structure directly enables future work on sensitivity, symbolic DC analysis, and behavioral optimization (all target Laplacian matrices).

## Confirmation

1. **Hand-computed examples**: Create reference MNA matrices for canonical circuits:
   - Single-stage VCCS (voltage-controlled current source): MNA matrix matches Vladimirescu *Computer Methods for Circuit Analysis and Design* Ch. 4.
   - Inductor netlist: `L1 1 2 1m` produces correct branch current variable and inductance stamp.
   - Transformer with mutual inductance: Coupled inductor stamp matches *Computer Methods* coupled-element formulation.
2. **Sparsity verification**: MNA matrix for a 1000-node circuit has sparsity profile matching expected topology (average degree ~4 neighbors per node).
3. **Index-1 DAE check**: Transient simulation of RC ladder and LC tank must converge with Radau IIA (confirming index-1 structure); failure would indicate formulation error.

## Pros and Cons of the Options

| Criterion | MNA | Pure Nodal | State-Space | Tableau |
|-----------|-----|-----------|-------------|---------|
| Handles V sources | ✓ native | ✗ no | ✗ no | ✓ yes |
| Handles inductors | ✓ native | ✗ no | ✓ direct | ✓ yes |
| Sparse structure | ✓ yes | ✓ yes | ✗ dense | ✗ dense |
| Matrix size | ~ n+k | ✓ n | ~ r | ✗ 2n+m |
| SPICE compatibility | ✓ yes | ✗ limited | ✗ no | ✗ no |
| Coupled elements | ✓ native | ✗ no | ✗ awkward | ✓ yes |
| Symbolic analysis | ✓ Laplacian | ~ medium | ✗ no | ~ cumbersome |
| Index-1 DAE | ✓ natural | ✓ natural | ✓ natural | ✗ index-2+ |
| Implementation effort | ~ medium | ✓ low | ✗ high | ✗ high |
| Device model fit | ✓ standard | ✗ no | ✗ no | ~ medium |

## Evidence

This decision is grounded in the following wiki evidence:
- [[vlsi-graph-methods]] — Graph-theoretic circuit analysis, Laplacian matrices, spanning trees, and topology-based optimization
- [[computer-methods-circuit-analysis-design]] — Vladimirescu's authoritative treatment of MNA, circuit stamps, and practical implementation
- [[differential-algebraic-equations]] — Index-1 DAE formulation, natural structure of circuit equations, and integration compatibility
- [[symbolic-circuit-analysis]] — Symbolic sensitivity analysis, behavioral optimization, and techniques targeting MNA matrices
