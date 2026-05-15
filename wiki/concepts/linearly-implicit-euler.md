---
title: "Linearly Implicit Euler"
type: concept
tags: [ode, dae, stiff, runge-kutta, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The linearly implicit Euler method is the simplest [[concepts/rosenbrock-method]] / W-method: y_{n+1} = y_n + h k_1 with (I − h J) k_1 = f(y_n), where J = f_y(y_n) (or an approximation). It is the linearisation of backward Euler at y_n: one step of Newton's method on the implicit Euler equation, starting from y_n.

## How It Works

For the [[concepts/dahlquist-test-equation]] y' = λ y the method gives y_{n+1}/y_n = 1/(1 − hλ) — exactly the backward-Euler stability function — so it is A-stable and L-stable. Order is 1 in general (classical Rosenbrock order conditions are trivially satisfied). The cost is one Jacobian evaluation, one LU, and one back-substitution per step. As the building block for stiff [[concepts/extrapolation-method]]s (SEULEX, Bader–Deuflhard EULSIM), the method has a perturbed asymptotic h-expansion that the Aitken–Neville tableau accelerates to high effective order. LIMEX (Deuflhard–Nowak) uses a [[concepts/quasilinear-dae]] variant of the same scheme.

## Key Parameters

- Jacobian J = f_y(y_n) (or stale / approximate variant for W-method).
- Step size h.
- Classical order 1; effective order in extrapolation up to ~ 12.

## When To Use

- Base scheme for stiff extrapolation codes.
- Quasilinear DAE integration via LIMEX.
- First-order benchmark / educational reference.

## Risks & Pitfalls

- Order 1 alone is rarely accurate enough; the method shines as the base of extrapolation, not as a standalone integrator.
- For very small ε in SPPs, the perturbed expansion has localised initial-layer terms — see [[concepts/perturbed-asymptotic-expansion]].
- Variable step + Jacobian update strategy materially affects efficiency.

## Related Concepts

- [[concepts/rosenbrock-method]]
- [[concepts/backward-euler]]
- [[concepts/extrapolation-method]]
- [[concepts/quasilinear-dae]]
- [[concepts/perturbed-asymptotic-expansion]]
- [[entities/seulex]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
