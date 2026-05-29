---
title: Butcher Simplifying Assumptions
type: claim
id: concepts/butcher-simplifying-assumptions
tags:
- ode
- numerical-integration
- runge-kutta
- order-conditions
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

Butcher's (1964) simplifying assumptions are three families of linear conditions on a Runge–Kutta tableau (A, b, c) that collapse the full set of order conditions onto a much smaller set:

- B(p): ∑_i b_i c_i^{k−1} = 1/k for k = 1, …, p (quadrature order).
- C(η): ∑_j a_{ij} c_j^{k−1} = c_i^k / k for k = 1, …, η, all i (stage / interpolation order).
- D(ζ): ∑_i b_i c_i^{k−1} a_{ij} = b_j (1 − c_j^k) / k for k = 1, …, ζ, all j (left-handed companion to C).

## How It Works

Butcher showed that B(p), C(η), D(ζ) together with p ≤ η + ζ + 1 and p ≤ 2η + 2 imply classical order p. The assumptions reduce the combinatorial explosion of tree-based order conditions to three polynomial relations, making it tractable to construct high-order implicit methods. C(η) is exactly the [[concepts/stage-order]] q; D(ζ) is its left-handed analogue. Gauss methods satisfy B(2s), C(s), D(s) (order 2s); [[concepts/radau-iia-method]] satisfies B(2s − 1), C(s), D(s − 1); [[concepts/lobatto-iiic-method]] B(2s − 2), C(s − 1), D(s − 1). The [[concepts/w-transformation]] expresses these as algebraic conditions on a Legendre basis.

## Key Parameters

- η — order of C (stage order).
- ζ — order of D.
- p — classical order achieved (≤ η + ζ + 1, ≤ 2η + 2).

## When To Use

- Constructing high-order implicit RK methods (Gauss, Radau, Lobatto families).
- Proving order theorems for collocation methods (where C(s) is automatic).
- Designing Rosenbrock methods with simplified tree-set conditions.

## Risks & Pitfalls

- The conditions are sufficient but not necessary; some methods of high order satisfy weaker forms.
- For [[concepts/explicit-runge-kutta]], C(η) is restrictive — explicit methods typically have q = 1.
- D(ζ) is harder to achieve than C(η) and accounts for many of the practical constraints in method construction.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stage-order]]
- [[concepts/collocation-method]]
- [[concepts/gauss-method]]
- [[concepts/radau-iia-method]]
- [[concepts/w-transformation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
