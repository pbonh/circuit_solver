---
title: W-Transformation
type: claim
id: concepts/w-transformation
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

The W-transformation (Hairer–Wanner 1981) is a change of basis on the Runge–Kutta stage space using the shifted Legendre polynomials evaluated at the nodes c_i: W_{ij} = P_{j−1}(c_i), with P_j the j-th shifted Legendre polynomial on [0, 1] normalised so ∫_0^1 P_j P_k = δ_{jk}/(2j+1). The transformed coefficient matrix Ã = W^{−1} A W has block-tridiagonal structure that makes order, stability, and algebraic-stability conditions much easier to analyse than in the original basis.

## How It Works

In the W-basis the [[concepts/butcher-simplifying-assumptions]] B(p), C(η), D(ζ) translate into tractable conditions on the first few rows / columns of Ã. The [[concepts/algebraic-stability]] matrix M reduces to a continued-fraction positivity check (Wanner 1980), which Hairer–Wanner use to verify B-stability of Gauss, Radau IA, Radau IIA, and Lobatto IIIC by inspection. The W-transformation is also the construction tool for new method families: choosing Ã with prescribed structure and inverting gives A with guaranteed order and stability.

## Key Parameters

- Node vector c.
- Legendre polynomial degrees up to s − 1.
- Transformed matrix Ã = W^{−1} A W.

## When To Use

- Designing high-order IRK methods with prescribed stability.
- Verifying algebraic stability via continued-fraction tests.
- Translating Butcher's tree-based order conditions into polynomial conditions.

## Risks & Pitfalls

- The transformation depends on the chosen polynomial basis; cross-family comparisons need consistent normalisation.
- For non-Lobatto / non-Radau nodes the structure of Ã is not block-tridiagonal, and the simplification is lost.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/algebraic-stability]]
- [[concepts/gauss-method]]
- [[concepts/radau-iia-method]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/collocation-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
