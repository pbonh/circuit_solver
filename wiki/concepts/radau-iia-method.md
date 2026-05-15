---
title: "Radau IIA Method"
type: concept
tags: [ode, numerical-integration, runge-kutta, stiff, dae, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The s-stage Radau IIA method is the [[concepts/collocation-method]] on the right-shifted Radau nodes — the s − 1 roots of P_{s−1}(2t − 1) + P_s(2t − 1) supplemented by c_s = 1. It has classical order 2s − 1, [[concepts/stage-order]] s, and is A-stable, L-stable, algebraically stable, B-stable, and [[concepts/stiffly-accurate-method]] by construction.

## How It Works

Inclusion of the right endpoint c_s = 1 forces stiff accuracy and R(∞) = 0; the choice of Radau quadrature gives the maximum order 2s − 1 compatible with this constraint. The [[concepts/stability-function]] is the (s − 1, s) sub-diagonal Padé approximation of e^z. The 3-stage Radau IIA (order 5) is the basis of Hairer–Wanner's RADAU5 code, which uses an eigendecomposition of A^{−1} (one real eigenvalue plus a complex conjugate pair) to reduce the per-step solve from (3n)×(3n) to one n × n real plus one n × n complex linear system (≈5× speed-up over the naive form), plus Hessenberg pre-transformation for dense Jacobians and Gustafsson predictive step control.

## Key Parameters

- Number of stages s; the popular variant is s = 3 (order 5).
- Nodes c_i with c_s = 1.
- A-, L-, B-, algebraic stability; R(∞) = 0.
- Stage order s, classical order 2s − 1.

## When To Use

- Stiff ODEs (the gold-standard implicit RK choice).
- Index-1 [[concepts/differential-algebraic-equation]]s and singular-perturbation problems.
- Index-2 DAEs with [[concepts/projected-runge-kutta]] augmentation.
- Smooth long-time integration where high order pays off.

## Risks & Pitfalls

- Still suffers [[concepts/order-reduction]] on stiff problems (effective order s, not 2s − 1, in the stiff component).
- Implementation is more involved than SDIRK because A is fully coupled.
- For very large dim(y), the dense Newton may be too expensive; consider Krylov-IRK variants or [[concepts/rosenbrock-method]].

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/l-stability]]
- [[concepts/algebraic-stability]]
- [[concepts/radau-ia-method]]
- [[concepts/gauss-method]]
- [[entities/radau5]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
