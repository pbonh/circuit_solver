---
title: BDF Methods
type: concept
slug: bdf-methods
created: 2026-06-16
updated: 2026-06-16
summary: Backward Differentiation Formulas — implicit linear multistep methods that are A(α)-stable for orders 1-6 and dominate stiff ODE/DAE solvers (Gear's method, DASSL, VODE, SPICE G2).
tags: [numerical-methods, bdf, multistep, stiff-ode, circuit-simulation, gear]
sources: [solving-ode-ii-stiff-dae]
status: active
---

# BDF Methods

Backward Differentiation Formulas are linear multistep methods of the form:

  sum_{j=0}^{k} α_j y_{n-j} = h β_0 f(t_n, y_n)

where only the current step's f value appears (no past f values). They approximate the derivative by a backward difference of past solution values, then match it to the RHS.

## Stability Properties

| Order | A-stable? | A(α)-stable? | Notes |
|---|---|---|---|
| 1 (Backward Euler) | Yes | Yes (α=90°) | L-stable; maximum damping |
| 2 (Gear2, BDF2) | Yes | Yes (α=90°) | Used in SPICE as "Gear2" |
| 3 | No | Yes (α≈86°) | Still stiffly stable in practice |
| 4 | No | Yes (α≈73°) | |
| 5 | No | Yes (α≈51°) | Borderline; variable-order codes switch adaptively |
| 6 | No | Yes (α≈18°) | Rarely used; marginal |
| ≥7 | No | No | Unstable on stiff problems |

**Second Dahlquist barrier**: No linear multistep method can be A-stable for order > 2. BDF methods bypass this via A(α)-stability — stable for arguments in a wedge around the negative real axis, sufficient for most stiff problems.

## Connection to SPICE

From [[integration-methods]] and [[simulation-analog-mixed-signal-circuits]]:
- SPICE's "Gear2" = BDF2 = the 2nd-order backward difference formula
- SPICE also uses Backward Euler (BDF1) and Trapezoidal Rule (not BDF; Adams-Moulton order 2)
- DASSL, VODE, CVODE extend BDF up to order 5 with variable stepsize/order control

## Implementation

**Predictor-corrector**: Explicit Adams predictor provides starting guess; BDF corrector iterates (or solves directly). Variable order: starts at BDF1 (Backward Euler), increases order as solution becomes smooth.

**Variable stepsize**: More complex than fixed-step BDF; require interpolation of past values. Nordsieck representation stores scaled derivatives rather than past solution values — simplifies variable-step implementation.

**Simplified Newton**: At each step, solve the implicit equation using Newton iterations with a lagged Jacobian (recomputed infrequently). This is essentially the SPICE NR iteration per timepoint.

**Starting**: Multistep methods require k past values to start; use Runge-Kutta or step-halved lower-order BDF for startup.

## Convergence for DAEs

BDF methods converge for index-1 [[differential-algebraic-equations]] but exhibit order reduction at the algebraic component (z converges at order k-1 vs. y at order k). Radau IIA (in [[stiff-ode-methods]]) avoids this order reduction.

## Why it matters

- BDF is the workhorse for stiff DAE in EDA: SPICE uses BDF1 and BDF2; industrial solvers use BDF up to order 5
- The second Dahlquist barrier explains why higher-order explicit methods cannot be used on stiff circuits
- Variable-order BDF (DASSL/CVODE strategy) adapts automatically to varying stiffness across a simulation

## Related concepts and entities

- [[stiff-ode-methods]] - Radau, Rosenbrock alternatives to BDF
- [[differential-algebraic-equations]] - the DAEs BDF solves
- [[integration-methods]] - SPICE integration context (BE = BDF1, G2 = BDF2)
- [[spice-simulation]] - uses BDF2 as its primary stiff solver
- [[circuit-simulation]] - primary application domain
- [[solving-ode-ii-stiff-dae]] - Hairer & Wanner: definitive reference on BDF, Radau, and DAE methods
