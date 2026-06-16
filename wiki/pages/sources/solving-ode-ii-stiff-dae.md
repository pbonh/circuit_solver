---
title: "Solving Ordinary Differential Equations II: Stiff and Differential-Algebraic Problems"
type: source
slug: solving-ode-ii-stiff-dae
created: 2026-06-16
updated: 2026-06-16
summary: Hairer & Wanner's definitive reference on numerical methods for stiff ODEs and DAEs — Runge-Kutta, BDF, Rosenbrock, and index reduction — with Fortran codes RADAU5, RODAS, SEULEX.
source_file: Books/solving_ordinary_differential_equations_ii
tags: [numerical-methods, stiff-ode, dae, runge-kutta, bdf, circuit-simulation, differential-algebraic]
status: active
---

# Solving Ordinary Differential Equations II: Stiff and Differential-Algebraic Problems

- **Source file:** `sources/Books/solving_ordinary_differential_equations_ii/`
- **Author / origin:** Ernst Hairer & Gerhard Wanner, Université de Genève
- **Date:** 1st ed. 1991; 2nd revised ed. 1996, corrected 2002. Springer Series in Computational Mathematics, vol. 14.

## Summary

The definitive graduate reference on numerical solution of stiff ODEs and differential-algebraic equations (DAEs). Provides rigorous theory, practical algorithms, and production-quality Fortran codes. Complements Volume I (non-stiff methods by Hairer, Nørsett, Wanner).

### Chapter IV: Stiff Problems — One-Step Methods

Motivating examples of stiffness: chemical reaction systems, electrical circuits (RC, transistor amplifiers), diffusion PDEs, highly oscillatory systems. The defining problem is that explicit methods (Forward Euler, explicit RK) require extremely small steps for stability when the circuit has time constants much shorter than the desired simulation window.

**Stability analysis**: A-stability (stability region covers the entire left half-plane), L-stability (A-stability + R(∞) = 0 for stiff decay), A(α)-stability. Padé approximations to the exponential are central to constructing high-order A-stable stability functions. Order stars classify rational approximations by order and stability.

**Implicit Runge-Kutta (IRK) methods**: Gauss (superconvergent, A-stable but not L-stable), Radau IA/IIA (L-stable, superconvergent; RADAU5 is the standard stiff solver), Lobatto IIIA/IIIB/IIIC. Implementation via simplified Newton iterations on the stage equations; W-transformation for efficient linear algebra.

**SDIRK (Singly Diagonally Implicit RK)**: Cheaper implementation (one LU per step); stiffly accurate SDIRK methods with R(∞)=0. Good for modest stiffness.

**Rosenbrock methods**: Linearize the nonlinear system at the beginning of each step; avoid iterative nonlinear solve; require exact Jacobian (W-methods allow inexact). RODAS is the flagship stiffly-accurate Rosenbrock code.

**Extrapolation methods**: Richardson extrapolation on the linearly implicit midpoint rule (SEULEX); smoothing required for stiff problems. Dense output for interpolation between accepted steps.

**Contractivity and B-stability**: One-sided Lipschitz condition; B-stability (bounded growth in the B-norm); algebraic stability (Butcher tableau conditions: M = B·A + A^T·B - b·b^T ≥ 0). Algebraically stable methods include Gauss, Radau IIA; SDIRK and Rosenbrock methods are generally not algebraically stable.

**B-convergence**: Order reduction phenomenon — IRK methods can lose order on very stiff problems; B-convergence characterizes the true order on stiff systems. Radau IIA achieves B-convergence of order 2s-1.

### Chapter V: Multistep Methods for Stiff Problems

**BDF (Backward Differentiation Formulas)**: The dominant multistep method for stiff ODE/DAE — used in Gear's method (SPICE Gear2 ≡ BDF2), DASSL, VODE. A(α)-stable for orders 1-6; A-stable only for orders 1-2 (second Dahlquist barrier). Predictor-corrector implementation. Variable stepsize/order algorithms.

**Second Dahlquist barrier**: Linear multistep methods cannot be A-stable beyond order 2. BDF sidesteps this via A(α)-stability (stable for arguments in a cone around the negative real axis).

**G-stability (one-leg methods)**: Dahlquist's generalization of B-stability to multistep methods. Equivalence: A-stability ↔ G-stability for one-leg methods.

**Order stars on Riemann surfaces**: Advanced stability theory for multistep methods — needed to analyze methods with complex arithmetic.

### Chapter VI: Singular Perturbation Problems and Index 1 Problems

Singular perturbation: ε y' = f(y, z), z' = g(y, z) with small ε. As ε→0, the fast component y is slaved to the algebraic constraint — yields a DAE of index 1. ε-embedding method: treat ε as homotopy parameter.

**Transistor amplifier** as running example: stiff ODE arising from a nonlinear circuit with widely differing RC time constants. Convergence of Runge-Kutta methods for index-1 systems proven via ε-expansion.

**Quasilinear problems**: C(y)y' = f(y) with singular or nearly singular C(y) (state-dependent mass matrix). Arises in moving finite element methods.

### Chapter VII: Differential-Algebraic Equations of Higher Index

**DAE index**: The differentiation index measures how many times the algebraic constraints must be differentiated to recover an ODE. Index-0 = pure ODE; index-1 = standard constraint; index-2 = velocity/constraint coupling; index-3 = position/velocity/acceleration (mechanical systems).

**Examples**: Modified Nodal Analysis (MNA) circuit equations are index-1 DAEs. Mechanical multibody systems (Newton-Euler with holonomic constraints) are index-3. Control problems.

**Index reduction**: Differentiate constraints to lower index; stabilize via projection onto constraint manifold or Baumgarte stabilization; local state space form. Overdetermined DAEs.

**Multistep methods for index-2 DAE**: BDF convergence proven with order reduction at the algebraic component. Simplified Newton iteration for large sparse systems.

**Runge-Kutta methods for index-2 DAE**: Radau IIA retains superconvergence at the y-component; z-component has lower order. Projected RK methods for invariant preservation.

**Symplectic methods for constrained Hamiltonian systems**: SHAKE and RATTLE algorithms; Lobatto IIIA-IIIB pair; backward error analysis on manifolds (modified Hamiltonian is preserved to high order).

**Fortran codes**: RADAU5 (5th-order Radau IIA; the gold standard for stiff ODE/index-1 DAE), RODAS (4th-order stiffly-accurate Rosenbrock), SEULEX (variable-order extrapolation). All with dense output and sparse Jacobian support.

## Key takeaways

- BDF methods (orders 1-6) are the workhorse for stiff DAEs — SPICE's Gear2 is BDF2; DASSL/CVODE use BDF up to order 5
- Radau IIA (RADAU5) outperforms BDF on very stiff problems due to algebraic stability and B-convergence
- Circuit MNA equations are index-1 DAEs; mechanical systems with constraints are index-2 or higher
- Index reduction by differentiation restores solvability but requires careful stabilization
- Rosenbrock methods (RODAS) are efficient when exact Jacobians are affordable and stiffness is moderate
- B-stability / algebraic stability is the nonlinear analogue of A-stability — required for reliable stiff solvers

## Pages updated from this source

- [[stiff-ode-methods]] - concept created (stiff ODE and one-step implicit methods)
- [[differential-algebraic-equations]] - concept created (DAEs, index, circuit connection)
- [[bdf-methods]] - concept created (BDF/Gear multistep methods)
- [[runge-kutta-methods]] - concept created (implicit RK, Radau, Rosenbrock)
- [[integration-methods]] - extended with stiff solver perspective
- [[circuit-simulation]] - stiff DAE connection noted
- [[overview]] - updated
