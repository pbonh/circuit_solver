---
title: "W-Method"
type: concept
tags: [ode, numerical-integration, runge-kutta, stiff, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A W-method is a Rosenbrock-type linearly implicit Runge–Kutta method (Steihaug–Wolfbrandt 1979) in which the Jacobian J in the matrix (I − h γ A) is replaced by an *arbitrary* approximation A. Unlike a [[concepts/rosenbrock-method]] (which requires the exact Jacobian for order conditions to hold), a W-method's order conditions are formulated on a larger tree set TW so they are satisfied regardless of A.

## How It Works

The price for tolerating an inexact A is that the order-condition tree set TW grows much faster than the Rosenbrock set: a fourth-order W-method needs more order conditions than a fourth-order Rosenbrock. The benefit is that A can be a low-cost approximation — a banded part of J, an old J from many steps ago, or even the identity — giving practical robustness on problems where computing or factorising the exact Jacobian is prohibitive. The W-method coefficient matrix decomposes (β = α + γ as in Rosenbrock); stability uses the same [[concepts/stability-function]] machinery with J replaced by A. ROW methods (Rosenbrock–Wanner) are a popular mid-point: enough order conditions for a usable order-4 method with embedded estimator, robust to mild J inaccuracies.

## Key Parameters

- Approximate Jacobian A.
- Tableau parameters (α, γ).
- Classical order p, stage order q.
- TW tree set size for order p.

## When To Use

- Stiff problems where the exact Jacobian is too expensive (large dense N, automatic differentiation unavailable).
- PDE method-of-lines discretisations where only a banded preconditioner is computed.
- Quasi-Newton-style integrators that reuse a stale J across many steps.

## Risks & Pitfalls

- More order conditions = more design constraints = fewer high-order practical methods.
- A too far from J degrades convergence and stability; in the worst case the method fails.
- The bound on inaccuracy ‖A − J‖ for which order p is preserved is method-dependent.

## Related Concepts

- [[concepts/rosenbrock-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/simplified-newton-iteration]]
- [[concepts/stability-function]]
- [[concepts/coercivity-coefficient]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
