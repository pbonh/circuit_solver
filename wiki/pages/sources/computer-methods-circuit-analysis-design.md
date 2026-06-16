---
title: "Computer Methods for Circuit Analysis and Design"
type: source
slug: computer-methods-circuit-analysis-design
created: 2026-06-16
updated: 2026-06-16
summary: Vladimirescu-era comprehensive treatment of computer circuit analysis — nodal/tableau formulations, sparse LU, sensitivity/adjoint methods, Newton-Raphson for nonlinear DC, BDF integration, optimization — the algorithmic foundation of SPICE-class simulators.
source_file: Books/Computer-Methods-for-Circuit-Analysis-and-Design
tags: [circuit-simulation, mna, sparse-matrix, newton-raphson, bdf, adjoint, sensitivity, optimization, spice]
status: active
---

# Computer Methods for Circuit Analysis and Design

- **Source file:** `sources/Books/Computer-Methods-for-Circuit-Analysis-and-Design/`
- **Author / origin:** [Wai-Kai Chen / Vladimirescu-era; Van Nostrand Reinhold]
- **Date:** ~1988 (classic EDA textbook)

## Summary

A comprehensive graduate-level treatment of the algorithms underlying SPICE-class circuit simulators. Covers circuit formulation (nodal, mesh, tableau, MNA), sparse matrix methods, sensitivity, transient integration, device modeling, and circuit optimization. This book is essentially the algorithmic reference for how SPICE works internally.

### Chapter 1-2: Fundamental Concepts and Network Equations

**Basic elements**: R, L, C, independent sources, transducers (dependent sources: VCCS, VCVS, CCCS, CCVS), two-port elements (admittance, impedance, chain, hybrid matrices). Thévenin/Norton equivalents, network scaling, poles/zeros, time-domain response via inverse Laplace.

**Nodal formulation**: Kirchhoff's current law → G·v = i. Nodal admittance matrix (NAM) for passive networks; extension to voltage-controlled transconductors (VCTs). Mesh formulation (dual). Gaussian elimination; triangular (LU) decomposition; pivoting strategies for numerical stability.

**Sparse matrix implementation**: Storage formats (column list, link list). Minimum degree ordering, Markowitz pivot selection. Fill-in minimization. Sparse LU factorization is the inner-loop bottleneck of every SPICE simulation — O(n^{1.5}) for planar circuits.

### Chapter 3: Graph-Theoretic Formulation

Circuit as oriented graph. Incidence matrix A (node-branch), cutset matrix Q, loop matrix B. KVL = B·v_b = 0, KCL = A·i_b = 0. Orthogonality: B·A^T = 0. Independent currents/voltages via spanning tree. Topological (tree-based) formulation of nodal and loop equations. State variable formulation (dynamic elements → state variables, algebraic elements → constraints).

### Chapter 4: General Formulation Methods

**Tableau formulation**: Complete network equation system — KCL, KVL, and branch constitutive relations in one augmented matrix. Block elimination reduces to familiar forms.

**Modified Nodal Analysis (MNA)**: Extend nodal to include voltage sources, inductors, mutual inductances, and controlled sources as additional variables. MNA = standard formulation in all modern simulators (SPICE, Spectre, Hspice). MNA matrix is the circuit Laplacian extended with element variables.

**Separate current/voltage graph formulation**: Topological MNA via two-graph theory — connects to [[advanced-symbolic-analysis-vlsi]].

### Chapter 5-6: Sensitivity

**Sensitivity**: ∂H/∂p for circuit performance H and component value p. Network function sensitivity via adjoint method: solve one extra linear system (the adjoint network) to compute sensitivities to ALL parameters simultaneously (O(1) extra cost per parameter vs. O(n) for finite difference).

**Adjoint method (Ch. 6)**: Adjoint of a linear network = transpose of the network matrix. Sensitivity of output to all inputs = one adjoint solve. Higher-order derivatives via repeated adjoint application. Large-change sensitivity (Woodbury formula for rank-1 updates — single component change without re-factoring).

**Fault analysis**: Detecting faulty components by measuring circuit response deviation. Related to Boolean test vector generation.

### Chapter 7: Network Functions in the Frequency Domain

Complex frequency domain analysis. Computer generation of network functions (rational polynomials in s). Unit circle polynomial interpolation for frequency response. Poles and zeros from system matrices (eigenvalue problem). Symbolic analysis algorithms (Section 8.5) — connection to [[symbolic-circuit-analysis]].

### Chapter 8: Large Change Sensitivity

Woodbury (matrix inversion lemma) for rank-1 changes. Differential sensitivity via H-matrix (hierarchical). Fault analysis. Zero pivots in sparse factorization. **Symbolic analysis section** (8.5): generating symbolic rational functions of component values — complement to numerical simulation.

### Chapter 9: Introduction to Numerical Integration

**Simple methods**: Forward Euler (explicit, unstable on stiff circuits), Backward Euler (implicit, L-stable), Trapezoidal Rule (Adams-Moulton order 2, marginally stable). Order of integration and truncation error. Stability analysis (A-stability, root condition). Time-domain solution of linear networks.

### Chapter 10: Numerical Laplace Transform Inversion

Bromwich contour numerical inversion (alternative to time-stepping for linear networks). Stepping algorithm (Stehfest, Weeks methods). Stability properties of the inversion algorithm.

### Chapter 11: Device Modeling

Diode models (Shockley + junction capacitance + series resistance). FET models (JFET, MOSFET — SPICE Level 1/2/3, Meyer capacitance model). Bipolar transistor models (Ebers-Moll, Gummel-Poon). Macromodels (behavioral opamp models). Approximate device models for timing simulation.

### Chapter 12: DC Solution of Networks

**Newton-Raphson (NR) for DC**: Iterative linearization around operating point; Jacobian (conductance matrix G_J) computed from device model derivatives. Convergence criteria (KCL residue check). Convergence aids: source ramping (related to [[homotopy-methods]]), Gmin stepping. DC sensitivity computation.

### Chapter 13: Numerical Integration of Differential and Algebraic-Differential Equations

**Linear multistep (LMS) formulae**: BDF family (BDF1 = Backward Euler, BDF2 = Gear2, BDF3-6). Adams family (Adams-Bashforth explicit, Adams-Moulton implicit). Theory: consistency, zero-stability, convergence. Properties: A-stability (BDF1-2), A(α)-stability (BDF3-6). **Variable step and order BDF** — the algorithm in SPICE's Gear solver.

**Tableau and MNA for nonlinear transient**: Augment MNA with time-discretized capacitor/inductor stamps; reduce to per-timepoint nonlinear algebraic system solved by NR. This is exactly what SPICE does at each timestep.

### Chapter 14: Digital and Switched-Capacitor Networks

z-Transform for discrete-time circuits. SC network formulation: switched capacitors → equivalent resistors for charge conservation analysis. Spectral analysis. Sample-hold inputs. **Symbolic analysis of SC networks** — transfer function in z-domain.

### Chapter 15-17: Optimization

**Optimization theory**: Unconstrained minimization (gradient, Newton, conjugate gradient, quasi-Newton BFGS). Line search algorithms. Constrained minimization (KKT conditions, penalty/barrier methods).

**Time-domain sensitivity and steady state**: Sensitivity of objective function over a transient. Steady-state analysis via sensitivity networks (for periodic circuits). Steady-state by extrapolation (related to harmonic balance).

**Design by minimization (Ch. 17)**: Least-squares objective, minimax (Chebyshev), sensitivity minimization, **Monte Carlo yield analysis** (yield = fraction of random parameter samples meeting specs). This is the original circuit optimization framework from which modern EDA tools descend.

**Appendices**: Laplace transforms, partial fractions, contour integration, full sparse matrix solver code (Fortran), sparse LU solver implementation, mathematical topics (Newton's method proofs, eigenvalue bounds).

## Key takeaways

- MNA is the correct formulation for SPICE — extends nodal to handle all linear elements including voltage sources and inductors
- Sparse LU factorization is the computational bottleneck; minimum-degree ordering minimizes fill-in; Markowitz criterion extends this to dynamic pivoting
- The adjoint method computes gradient of any scalar objective w.r.t. ALL component values at cost of ONE extra linear solve — the foundation of circuit optimization and yield analysis
- BDF methods (Ch. 13) are the correct family for stiff circuit transient — connects to [[bdf-methods]] and [[integration-methods]]
- Monte Carlo yield analysis (Ch. 17) is the original statistical circuit analysis method — connects to [[advanced-symbolic-analysis-vlsi]] GPU Monte Carlo
- Device models in Ch. 11 are the textbook versions of SPICE LEVEL1-3 models — ground truth for [[mosfet-physics]] and [[pn-junction]]

## Pages updated from this source

- [[circuit-simulation]] - MNA, NR, BDF formulation detailed
- [[spice-simulation]] - algorithmic foundation completed
- [[newton-raphson]] - adjoint method, DC convergence extended
- [[integration-methods]] - LMS family, BDF variable step/order added
- [[bdf-methods]] - variable step/order BDF circuit application
- [[differential-algebraic-equations]] - MNA as DAE confirmed
- [[vlsi-graph-methods]] - MNA/graph-theoretic formulation linked
- [[symbolic-circuit-analysis]] - Ch. 8 symbolic analysis noted
