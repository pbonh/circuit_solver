---
title: "Transpose System (Adjoint) Sensitivity Method"
type: concept
tags: [sensitivity, foundational, well-established, sparse-matrix]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt"]
confidence: high
---

## Definition

The transpose-system method computes the sensitivity of a scalar output phi = d^T X with respect to many parameters by introducing an adjoint vector X^a that solves T^T X^a = -d. The sensitivity is then d phi / d h_i = (X^a)^T [-(d T/d h_i) X + d W/d h_i]. Only TWO solves are needed (direct and adjoint), regardless of the number of parameters.

## How It Works

1. Solve TX = W for X by LU factorization (factorize T = LU).
2. Solve T^T X^a = -d for X^a, reusing the same L and U via the SOLVET routine (forward sub with U^T, back sub with L^T).
3. For each parameter h_i, evaluate d phi/d h_i = (X^a)^T (-(dT/dh_i) X + dW/dh_i). With nodal/MNA/tableau stamps, this reduces to a single inner product (x_p^a - x_q^a)(x_r - x_s) per element, costing at most one multiply-subtract.

The method is mathematically equivalent to the Tellegen-theorem-based adjoint network approach but extends naturally to non-graph formulations (two-graph MNA, etc.).

## Key Parameters

- Matrix size n.
- Number of parameters (no factorization cost per parameter — only the cheap product evaluation).
- Output dimension k: this method handles one scalar output at a time; k outputs need k adjoint solves.

## When To Use

- Optimization-based design where the gradient of a scalar objective is needed with respect to many element values.
- Tolerance/parasitic analyses with many parameters but one output of interest.
- Default sensitivity method in any production CAD tool.

## Risks & Pitfalls

- One adjoint solve per output — costly when many outputs are desired.
- Generalized outputs (functions of X and h) require a more complex adjoint with RHS depending on X.

## Related Concepts

- [[concepts/sensitivity-network-method]]
- [[concepts/adjoint-method]]
- [[concepts/tellegen-theorem]]
- [[concepts/sensitivity-analysis]]
- [[concepts/lu-decomposition]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
