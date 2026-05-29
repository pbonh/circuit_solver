---
title: Higher-Order Sensitivity (Second Derivatives)
type: claim
id: concepts/higher-order-sensitivity
tags:
- sensitivity
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Higher-order sensitivities are second and higher mixed partial derivatives of a network output with respect to element parameters. The second-derivative formula in Vlach & Singhal Eq. 6.6.3 is:

d^2 phi/(d h_p d h_q) = (X^a)^T [(d^2 T/(d h_p d h_q)) X + (dT/d h_p)(dX/d h_q) + (dT/d h_q)(dX/d h_p)].

## How It Works

Computation requires the adjoint solution X^a (one solve) plus the direct sensitivity vectors dX/dh_p and dX/dh_q (one solve per parameter via the sensitivity-network method). The trade-off: with N_p parameters, this needs N_p extra solves for the first-order vectors. Alternatively, with n algebraic dimensions, one can compute the n-dimensional adjoint for each component of X and recover the same information; this is preferable when N_p > n.

## Key Parameters

- Matrix dimension n.
- Number of parameters N_p.
- Required output dimension.

## When To Use

- Newton-based optimization where the Hessian is required.
- Wiggling analyses where second-order distortion of the response is studied.
- Trust-region methods that need both gradients and curvature.

## Risks & Pitfalls

- Storage of dX/dh_p vectors can be substantial.
- Mixed partials are symmetric (d^2 phi / (d h_p d h_q) = d^2 phi / (d h_q d h_p)); exploiting this halves work.
- Round-off errors compound; double-precision arithmetic is essential.

## Related Concepts

- [[concepts/transpose-system-method]]
- [[concepts/sensitivity-network-method]]
- [[concepts/optimization-theory]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
