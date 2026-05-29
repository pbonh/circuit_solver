---
title: Half-Explicit Method
type: claim
id: claim-half-explicit-method
tags:
- dae
- mechanical
- runge-kutta
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

A half-explicit method (Hairer–Lubich–Roche 1989; Brasey–Hairer 1993; Murua 1995; Arnold 1995) is a numerical integrator for index-2 / index-3 DAEs in which the differential variable y is advanced *explicitly* by a Runge–Kutta step while the algebraic variable z (or the Lagrange multiplier λ) is determined *implicitly* by enforcing the algebraic constraint at each stage: 0 = g(Y_i).

## How It Works

For [[concepts/constrained-mechanical-system]]s the per-stage linear system is the saddle-point system (M, G^T; G, 0) (Δu, λ) = (h f − M(u_{n,i} − u_n), 0), Eq. 6.17 in Hairer–Wanner. This is just one linear-system solve per stage — far cheaper than a fully implicit RK on the same system. Coupling the half-explicit framework with Dormand–Prince RK5(4) pairs gives Murua's order-5 [[entities/phem56|PHEM56]] code, dominant in the nonstiff regime of multibody benchmarks (Andrews [[concepts/squeezer-mechanism]]). The GBS-type extrapolation (Lubich 1989, Eq. 6.18) achieves h² expansion for index-2 problems whose f is linear in z.

## Key Parameters

- Underlying explicit RK method (DOPRI, Verner, …).
- Per-stage saddle-point linear-system solver.
- Index of the DAE being treated (2 or 3).

## When To Use

- Multibody dynamics in the nonstiff regime (vehicle simulation, biomechanics).
- Constrained mechanical systems where Jacobian-LU cost dominates the budget.
- Index-2 DAEs with linear-in-z right-hand sides.

## Risks & Pitfalls

- Useless on stiff DAEs — explicit RK part loses stability.
- Saddle-point linear systems can be ill-conditioned near constraint singularities.
- Index-3 variants need extra projection or velocity-level enforcement.

## Related Concepts

- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/runge-kutta-method]]
- [[concepts/explicit-runge-kutta]]
- [[concepts/projected-runge-kutta]]
- [[concepts/multibody-system]]
- [[entities/phem56]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
