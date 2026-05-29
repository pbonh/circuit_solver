---
title: Dahlquist Test Equation
type: claim
id: claim-dahlquist-test-equation
tags:
- ode
- numerical-integration
- stability
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

The Dahlquist test equation is the scalar linear autonomous ODE y' = λy with λ ∈ ℂ. Introduced by Dahlquist (1956, 1963) to isolate the linear stability behaviour of a numerical method, it is the canonical model on which [[concepts/a-stability]], [[concepts/l-stability]], [[concepts/a-alpha-stability]], [[concepts/ao-stability]], and [[concepts/stability-region]] are all defined.

## How It Works

Applying any one-step method to y' = λy produces y_{n+1} = R(hλ) y_n where R(z) is the method's [[concepts/stability-function]] — a polynomial for explicit RK, a rational function for implicit RK and Rosenbrock methods. For [[concepts/linear-multistep-methods]] the analogue is the characteristic equation ρ(ζ) − μσ(ζ) = 0 with μ = hλ, and the [[concepts/root-locus-curve]] is the image of |ζ| = 1. Stability of the numerical solution reduces to the inequality |R(hλ)| ≤ 1 (or |ζ| ≤ 1 for all roots in the multistep case). The diagonalisation argument extends this scalar analysis to any constant-coefficient linear system whose Jacobian eigenvalues lie in the [[concepts/stability-region]] when scaled by h.

## Key Parameters

- λ ∈ ℂ (Jacobian eigenvalue of the linearised system).
- h > 0 (step size).
- Product hλ ∈ ℂ is the actual stability variable.

## When To Use

- Linear stability classification of any numerical ODE integrator.
- First-line analysis before extending to nonlinear or non-autonomous problems (then upgraded to [[concepts/an-stability]], [[concepts/b-stability]], or [[concepts/g-stability]]).
- Constructing the [[concepts/stability-region]] / [[concepts/stability-domain]] of a method.

## Risks & Pitfalls

- The scalar test equation cannot detect order reduction on stiff nonlinear problems — see [[concepts/b-convergence]], [[concepts/order-reduction]].
- Real Jacobian spectra alone (covered by [[concepts/ao-stability]]) miss oscillatory stability failures.
- Linear analysis cannot certify behaviour on quasi-linear / non-autonomous / coupled systems without additional structure.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/l-stability]]
- [[concepts/stability-function]]
- [[concepts/stability-region]]
- [[concepts/root-locus-curve]]
- [[concepts/an-stability]]
- [[concepts/dahlquist-barrier]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
