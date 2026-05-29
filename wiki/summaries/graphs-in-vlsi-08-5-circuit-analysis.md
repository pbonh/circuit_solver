---
title: 'Graphs in VLSI — Chapter 5: Circuit Analysis'
type: source
id: source-graphs-in-vlsi-08-5-circuit-analysis
kind: derived-summary
tags:
- vlsi
- circuit
- analysis
- graph
- sparse-matrix
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
---

## Key Points

- Modern Modified Nodal Analysis (MNA) was first presented in 1975. MNA represents linear and nonlinear circuits as a system G̃ x + C̃ ẋ = b where G̃ encodes conductances and voltage-source incidences and C̃ encodes capacitances and inductances. Companion models replace transient elements (capacitors, inductors) with equivalent conductance + current source pairs at each time step.
- The historical lineage of circuit simulators: TAP (1959) → NET1 (1963) → SCEPTRE (1967) → CIRPAC → ASTAP (1971, sparse matrix and variable time step) → CANCER, SLIC (early 1970s) → SPICE (1973) → SPICE2 (1975, MNA-based, becomes worldwide standard).
- Backward Euler discretization of a capacitor gives i_C(t^k) = g_eq v_C(t^k) + i_eq with g_eq = C/h and i_eq = -C/h · v_C(t^{k-1}), where h is the time step.
- Voltage sources break symmetric positive definiteness (SPD); LU factorization (instead of Cholesky) is then required. Norton-equivalent conversions can restore SPD.
- Direct linear solvers (LU, Cholesky) are infeasible for huge VLSI matrices. Iterative methods are used: stationary methods (Jacobi, Gauss-Seidel, SOR), and non-stationary Krylov-subspace methods (CG, BICG, BICGSTAB, MINRES, GMRES). Preconditioning with split matrices, incomplete LU/Cholesky, or SParse Approximate Inverse (SPAI) accelerates convergence.
- Domain Decomposition (DD) partitions the circuit graph into m subgraphs plus an interface graph G_0, producing an "arrowhead" block matrix. Solving local systems A_i x_i + E_i x_0 = b_i in parallel followed by a small interface solve is much cheaper than solving the full system. Schwarz / overlapping-domain methods (used for a 192-million-node power grid in 5 minutes on 1200 processors) further accelerate by allowing partition overlap.
- Hierarchical matrices (H-matrices) hierarchically cluster the matrix into blocks; full-rank diagonal blocks remain dense while off-diagonal rank-deficient blocks are stored as factored products A_{i,j} = M N^T. LU factorization complexity drops from O(n^3) to O(n (log n)^2); applied to partial-element-equivalent power-supply analysis with 4-orders-of-magnitude speedup.
- Multigrid methods (Fedorenko 1960s; formalized by Brandt 1970s and Hackbusch 1970s) use three operations: smoothing (a few iterations of an iterative solver), restriction (coarsen the grid), and prolongation (interpolate back to the fine grid). Common cycle structures are V-cycle (fast), W-cycle, and F-cycle (most robust). Geometric multigrids exploit regular layouts; Algebraic multigrid (AMG, e.g., PowerRush) achieves O(|V|) complexity on irregular grids — 38-million-node DC analysis in 169 seconds.
- Non-MNA techniques include S parameters for black-box components, random walks (commute time corresponds to effective resistance; error scales as 1/√M_experiments), and infinite lattice analysis (closed-form effective resistance via Green's functions due to McCrea, Whipple 1940; van der Pol 1933; Spitzer 1976; Cserti 2000). Anisotropic infinite-grid resistance was derived by Bairamkulov as part of this book (Eq. 5.73). IR drop analysis can run in constant time per node assuming nodes are far from grid boundaries.

## Relevant Concepts

- [[concepts/modified-nodal-analysis]] — the dominant circuit-analysis matrix formulation.
- [[concepts/laplacian-matrix]] — central to MNA's conductance matrix.
- [[concepts/sparse-matrix]] — practical VLSI MNA matrices are sparse and benefit from specialized solvers.
- [[concepts/companion-model]] — transient element replaced by resistor + source at each time step.
- [[concepts/conjugate-gradient-method]] — iterative solver for SPD systems.
- [[concepts/krylov-subspace-method]] — generalization including CG, GMRES, BICGSTAB.
- [[concepts/preconditioning]] — accelerates iterative solver convergence.
- [[concepts/domain-decomposition]] — divide-and-conquer partitioning of circuit graphs.
- [[concepts/hierarchical-matrix]] — H-matrix block-rank-deficient compression.
- [[concepts/multigrid-method]] — smoothing + restriction + prolongation for fast solving.
- [[concepts/algebraic-multigrid]] — multigrid without geometric regularity.
- [[concepts/scattering-parameters]] — black-box frequency-domain network characterization.
- [[concepts/random-walk]] — stochastic equivalent of effective resistance.
- [[concepts/effective-resistance]] — graph metric central to power-grid analysis.
- [[concepts/lattice-graph]] — infinite-mesh model for regular VLSI structures.
- [[concepts/lattice-greens-function]] — closed-form expression for infinite-grid effective resistance.
- [[entities/spice]] — descendent of CANCER, made MNA the standard.
- [[concepts/power-distribution-network]] — primary VLSI application of these methods.

## Source Metadata

- Source type: book chapter
- Book title: Graphs in VLSI
- Chapter: 5 — Circuit analysis
- File path: `raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
