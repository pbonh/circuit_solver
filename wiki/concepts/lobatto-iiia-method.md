---
title: Lobatto IIIA Method
type: claim
id: concepts/lobatto-iiia-method
tags:
- ode
- numerical-integration
- runge-kutta
- symplectic
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The s-stage Lobatto IIIA method is the [[concepts/collocation-method]] on Lobatto nodes — c_1 = 0, c_s = 1, with the s − 2 interior nodes given by the roots of the derivative of the shifted Legendre polynomial of degree s − 1. It has classical order 2s − 2, stage order s, and is A-stable but *not* L-stable (R(∞) = (−1)^{s−1}).

## How It Works

The inclusion of both endpoints makes Lobatto IIIA naturally symmetric, hence well suited as the position-update partner in the [[concepts/lobatto-iiia-iiib-pair]] symplectic DAE integrator. It is *not* algebraically stable (M is not non-negative definite) because the first weight b_1 = 0; consequently it is not B-stable and not B-convergent. The 2-stage Lobatto IIIA is exactly the trapezoidal rule.

## Key Parameters

- Number of stages s ≥ 2.
- Endpoints c_1 = 0, c_s = 1.
- Order 2s − 2, stage order s.
- R(∞) = (−1)^{s−1}.

## When To Use

- Symmetric (time-reversible) integration of smooth ODEs.
- Position update in [[concepts/lobatto-iiia-iiib-pair]] symplectic integrators for constrained Hamiltonian systems.
- Boundary-value problems via collocation.

## Risks & Pitfalls

- Not L-stable; oscillates on very stiff modes.
- Not B-stable; nonlinear stiff problems can amplify.
- Severe [[concepts/order-reduction]] on stiff problems and DAEs.

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/lobatto-iiib-method]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/trapezoidal-rule]]
- [[concepts/symplectic-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
