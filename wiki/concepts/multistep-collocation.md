---
title: "Multistep Collocation"
type: concept
tags: [ode, numerical-integration, multistep, runge-kutta, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A multistep collocation method (Guillou–Soulé 1969, Lie–Nørsett 1989) constructs a degree-(s + k − 1) spline polynomial that fits k past values y_{n−1}, …, y_{n−k} and s [[concepts/collocation-method|collocation]] conditions on f at nodes c_1, …, c_s in the current step. The endpoint y_{n+1} is the value of the spline at x_{n+1}.

## How It Works

The combination of k history points and s collocation conditions gives 2s + k − 2 order conditions; with Krylov's choice of c_i (roots of a specific orthogonal polynomial), the method achieves order p = 2s + k − 2. For k = 1, 2 the methods are A-stable (orders 5 and 6) and for k ≥ 3 they are A(α)-stable up to order p = 20. The method's stage equations are an IRK-flavoured implicit system of size s; the history dependence makes them multistep. Multistep collocation is a bridge between the [[concepts/runge-kutta-method]] world (high stage order) and the [[concepts/linear-multistep-methods]] world (use of history to save evaluations).

## Key Parameters

- Step history k.
- Number of collocation nodes s.
- Order p = 2s + k − 2.
- Stability sector α (depends on k, s).

## When To Use

- High-order stiff integration where the BDF order limit is too restrictive.
- Theoretical exploration of hybrid LMS / RK design.
- Singular-perturbation problems where stage order matters (s = stage order).

## Risks & Pitfalls

- Variable-step implementation is intricate; the history-polynomial maintenance is non-trivial.
- Practical codes are less mature than for plain BDF or IRK.

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/linear-multistep-methods]]
- [[concepts/runge-kutta-collocation]]
- [[concepts/gear-bdf]]
- [[concepts/general-linear-method]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
