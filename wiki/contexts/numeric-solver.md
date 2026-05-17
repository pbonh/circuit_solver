---
title: "Numeric Solver"
type: context
tags: [solver, sparse-matrix, circuit-solver, bounded-context]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "concepts/modified-nodal-analysis"
  - "concepts/newton-raphson-method"
  - "concepts/sparse-matrix"
  - "concepts/lu-decomposition"
  - "concepts/integration-method"
confidence: high
---

## Model

The numeric-solver context owns the mathematical solution of circuit equations. Core entities:
- `MNAMatrix` — the modified nodal analysis matrix (G + sC) assembled from element stamps.
- `SparseMatrix` — the CSR/CSC/COO storage and direct-factorization engine.
- `LinearSolver` — the sparse LU or Cholesky direct solver with fill-reducing ordering.
- `NewtonRaphsonSolver` — the iterative nonlinear root finder that builds and solves linear updates.
- `IntegrationMethod` — the finite-difference formula (Backward Euler, Trapezoidal, Gear BDF) used in transient analysis.
- `CompanionModel` — the discrete-time resistive equivalent of a dynamic element at a given timestep.
- `LocalTruncationError` — the per-timestep error estimate used to accept or reject a step.

Key invariants: The MNA matrix is structurally nonsingular (after ground suppression and proper MNA augmentation). The linear solver produces a solution whose residual is below the pivoting tolerance. Newton-Raphson updates decrease both residue and update norms when close to a solution. The integration method is stiffly stable for the circuits under test.

## Boundary

- Starts at assembled element stamps and device linearizations.
- Ends at solution vectors (node voltages, branch currents, small-signal transfer functions) delivered to analysis-orchestration.
- Adjacent contexts:
  - `device-modeling` provides `LinearizedModel` stamps.
  - `analysis-orchestration` requests solves (DC, linear AC, transient timestep) and receives solution vectors.
- Artifacts crossing the boundary: `SolutionVector`, `JacobianMatrix`, `ConvergenceStatus`, `TimestepProposal`.

## Ubiquitous Language

- `MNA` — Modified Nodal Analysis, the standard matrix formulation.
- `SparseMatrix` — a matrix stored in a compressed sparse format.
- `LU` — lower-upper triangular factorization.
- `Fill-In` — nonzeros introduced during factorization that were zero in the original matrix.
- `Ordering` — the permutation applied to reduce fill-in.
- `NewtonIteration` — one step of the [[concepts/newton-raphson-method]].
- `Residual` — the mismatch vector f(x) in the nonlinear equation.
- `Jacobian` — the derivative matrix J(x) of the nonlinear system.
- `Convergence` — the state where both update and residual norms are below tolerance.
- `Tolerance` — the user-defined or default threshold for convergence.
- `CompanionModel` — the discrete-time equivalent of a capacitor or inductor.
- `Stiffness` — the property of a circuit with widely separated time constants.
- `Timestep` — the discretization interval h in transient analysis.
- `LTE` — Local Truncation Error, the per-step discretization error estimate.

## Relationships

- [[context-maps/circuit-solver]]

## Architecture

- [[architecture/circuit-solver]] — C4 diagrams showing the numeric-solver context as the MNA assembly and sparse-LU engine.
