---
title: Differential-Algebraic Equations
type: concept
slug: differential-algebraic-equations
created: 2026-06-16
updated: 2026-06-16
summary: Systems mixing ODEs with algebraic constraints; arise in circuit simulation (MNA), mechanical systems, and control; characterized by their differentiation index.
tags: [dae, circuit-simulation, numerical-methods, mna, index]
sources: [solving-ode-ii-stiff-dae, simulation-analog-mixed-signal-circuits]
status: active
---

# Differential-Algebraic Equations

A DAE is a system F(t, y, y') = 0 that mixes differential equations with algebraic constraints. Unlike a pure ODE, not all variables have time-derivative terms, so initial conditions must satisfy hidden consistency conditions, and standard ODE solvers fail or degrade.

## The Differentiation Index

The **differentiation index** is the minimum number of times the DAE must be differentiated with respect to time to obtain an ODE. It measures how far the system is from an explicit ODE:

| Index | Description | Example |
|---|---|---|
| 0 | Pure ODE (trivially) | Standard IVP |
| 1 | One differentiation recovers ODE | Circuit MNA, chemical equilibria |
| 2 | Velocity constraints; twice for ODE | Mechanical systems (velocity level) |
| 3 | Acceleration constraints | Euler-Lagrange with holonomic constraints |

**Perturbation index** is an alternative measure based on how much perturbations in the residual amplify into solution errors; relevant for numerical analysis.

## Circuit Connection: Modified Nodal Analysis (MNA)

Circuit equations from MNA are index-1 DAEs (or index-0 when written in fully explicit form). The algebraic constraints come from voltage-source branch currents (KVL constraints with no capacitor equivalent). The "floating" inductor problem — a pure inductor loop — raises the index to 2.

From [[simulation-analog-mixed-signal-circuits]]: the simulator formulates "nonlinear first-order differential/algebraic equations" — this is precisely an index-1 DAE in MNA form. The stiffness arises from widely differing RC time constants.

## Index Reduction

Higher-index DAEs must be reduced before numerical integration:

1. **Differentiation**: Differentiate algebraic constraints until an ODE appears; risks numerical drift from constraint violation
2. **Projection**: After each step, project solution back onto the constraint manifold
3. **Baumgarte stabilization**: Add damped constraint terms to stabilize drift
4. **Local state space form**: Identify dependent/independent variables at each point
5. **Overdetermined systems**: More constraints than unknowns — requires special treatment

## Singular Perturbation View

A stiff ODE ε y' = f(y, z), z' = g(y, z) with ε → 0 becomes an index-1 DAE (0 = f(y, z), z' = g(y, z)). This motivates ε-embedding methods — treating ε as a homotopy parameter mirrors [[homotopy-methods]] in circuit simulation. The transistor amplifier in Hairer & Wanner is a canonical example.

## Symplectic Structure and Constraints

Mechanical systems with holonomic constraints form index-3 DAEs. Symplectic integrators (SHAKE, RATTLE, Lobatto IIIA-IIIB pair) preserve the constraint manifold structure and a nearby Hamiltonian (backward error analysis), making them superior for long-time dynamics.

## Why it matters

- Every practical circuit simulator solves an index-1 DAE; understanding index explains why floating nodes (index > 1) cause convergence failure
- BDF methods (Gear, DASSL) converge for index-1 DAE; convergence order reduces at the algebraic component
- Radau IIA (RADAU5) is more robust than BDF for DAE due to algebraic stability and stiff accuracy

## Related concepts and entities

- [[stiff-ode-methods]] - solvers for stiff ODEs and index-1 DAEs
- [[bdf-methods]] - BDF used in SPICE for stiff DAEs
- [[integration-methods]] - SPICE-level integration methods
- [[newton-raphson]] - nonlinear solver within DAE step computation
- [[spice-simulation]] - formulates circuit as DAE (MNA)
- [[circuit-simulation]] - primary application
