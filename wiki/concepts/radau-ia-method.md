---
title: Radau IA Method
type: claim
id: claim-radau-ia-method
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
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

The s-stage Radau IA method is the [[concepts/collocation-method]] on the left-shifted Radau nodes — c_1 = 0 plus the s − 1 roots of P_{s−1}(2t − 1) − P_s(2t − 1). Like [[concepts/radau-iia-method]], it has classical order 2s − 1, stage order s, and is A-stable, L-stable, and algebraically stable, but the left endpoint inclusion makes it the *left-handed* companion: column constraint replaces the right-end row constraint.

## How It Works

Radau IA satisfies the D(s) (left-handed) simplifying assumption rather than C(s) (right-handed). It is *not* stiffly accurate (the output b^T is not equal to the last row of A), so R(∞) ≠ 0 in the same automatic way as Radau IIA; nevertheless L-stability holds via the Padé-approximation argument and the careful node placement. The method is algebraically stable (Burrage–Butcher 1979), hence B-stable.

## Key Parameters

- Number of stages s.
- Nodes c_i with c_1 = 0.
- Order 2s − 1, stage order s.
- A-, L-, B-, algebraically stable.

## When To Use

- Stiff problems where a left-handed companion to Radau IIA is theoretically natural.
- Backward / time-reversed integration where the c_1 = 0 anchor is preferred.
- Construction of [[concepts/lobatto-iiia-iiib-pair]] families via combined left/right Radau.

## Risks & Pitfalls

- Not stiffly accurate; Radau IIA is usually preferred for DAEs and singular-perturbation problems.
- Same [[concepts/order-reduction]] limitation as all IRK methods on stiff problems.

## Related Concepts

- [[concepts/radau-iia-method]]
- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/algebraic-stability]]
- [[concepts/l-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
