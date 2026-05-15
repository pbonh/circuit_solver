---
title: "Runge–Kutta Method"
type: concept
tags: [ode, numerical-integration, runge-kutta, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A Runge–Kutta method is a one-step ODE integrator defined by a Butcher tableau (A, b, c) with A ∈ ℝ^{s×s}, b, c ∈ ℝ^s. It advances the solution of y' = f(x, y) from y_n to y_{n+1} via s internal stages Y_i = y_n + h ∑_j a_{ij} f(x_n + c_j h, Y_j), then y_{n+1} = y_n + h ∑_i b_i f(x_n + c_i h, Y_i).

## How It Works

When A is strictly lower triangular the method is *explicit* ([[concepts/explicit-runge-kutta]]); otherwise it is implicit ([[concepts/implicit-runge-kutta]]) and each step requires solving a nonlinear system in s · dim(y) unknowns. Diagonal cases ([[concepts/dirk-method]], [[concepts/sdirk-method]]) keep the implicit cost manageable. Order conditions come from rooted trees (Butcher 1963) and can be tackled directly or via [[concepts/butcher-simplifying-assumptions]] B(p), C(η), D(ζ). Stability is governed by the [[concepts/stability-function]] R(z) = 1 + z b^T (I − z A)^{−1} 𝟙. Special families optimise particular trade-offs: Gauss (max order 2s, A-stable), Radau IA / IIA (order 2s − 1, L-stable), Lobatto IIIA / IIIB / IIIC (order 2s − 2, varied stability properties), Rosenbrock (linearly implicit), DOPRI5 / DOPRI8 (embedded explicit pairs).

## Key Parameters

- Number of stages s.
- Tableau (A, b, c).
- Order p, stage order q, R(z).
- Embedded estimator (b̂, p̂) for error control.

## When To Use

- General-purpose ODE integration with adaptive step control.
- Stiff problems (use implicit RK, especially [[concepts/radau-iia-method]] or [[concepts/sdirk-method]]).
- Non-stiff problems (use [[concepts/explicit-runge-kutta]] like DOPRI5 / DOPRI8).
- DAEs and singular-perturbation problems (use [[concepts/stiffly-accurate-method]] IRK).
- Hamiltonian / mechanical systems (use [[concepts/symplectic-method]] like Gauss or Lobatto IIIA-IIIB pairs).

## Risks & Pitfalls

- High classical order p does not imply high effective order on stiff problems — see [[concepts/order-reduction]] and [[concepts/stage-order]].
- Implicit methods need a robust nonlinear solver ([[concepts/simplified-newton-iteration]]); convergence depends on [[concepts/coercivity-coefficient]] vs. the one-sided Lipschitz constant.
- Stability region size does not guarantee good nonlinear behaviour; consult [[concepts/b-stability]] / [[concepts/algebraic-stability]].

## Related Concepts

- [[concepts/explicit-runge-kutta]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/stability-function]]
- [[concepts/collocation-method]]
- [[concepts/rosenbrock-method]]
- [[concepts/symplectic-method]]
- [[concepts/w-transformation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
