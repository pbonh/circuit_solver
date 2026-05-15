---
title: "Logarithmic Norm"
type: concept
tags: [ode, numerical-integration, nonlinear, linear-algebra, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The logarithmic norm (also called logarithmic derivative or Dahlquist's μ) of a matrix A in a given vector norm is μ(A) = lim_{h→0^+} (‖I + h A‖ − 1) / h. It measures the maximal logarithmic growth rate of solutions to y' = A y in that norm, and can be negative when A is dissipative — unlike the operator norm ‖A‖, which is always non-negative.

## How It Works

For the Euclidean norm, μ_2(A) equals the largest eigenvalue of the symmetric part (A + A^T)/2. For ‖·‖_∞, μ_∞(A) = max_i (a_{ii} + ∑_{j≠i} |a_{ij}|), and for ‖·‖_1 analogously column-wise. The continuous contraction estimate is ‖e^{Ah}‖ ≤ e^{μ(A) h}; the nonlinear analogue is the [[concepts/one-sided-lipschitz-condition]] with ν = sup_x μ(f_y(x)). In Chapter VI of Hairer–Wanner, μ(g_z) ≤ −1 is the standard stability hypothesis for [[concepts/singular-perturbation-problem]]s ensuring the algebraic component contracts.

## Key Parameters

- Choice of norm (Euclidean, ∞, 1, weighted).
- μ(A) — can be negative, zero, or positive.
- For nonlinear f, sup_x μ(f_y(x)) bounds the one-sided Lipschitz constant.

## When To Use

- Stability proofs for linear and nonlinear ODEs.
- Estimating contraction rates on stiff problems.
- Hypothesis check for boundary-layer convergence theorems (μ(g_z) ≤ −1 implies exponential decay).
- Tighter alternative to the spectral abscissa for non-normal matrices.

## Risks & Pitfalls

- μ is norm-dependent: μ_2 ≤ μ_∞ in general, and they can differ in sign.
- Subadditivity μ(A + B) ≤ μ(A) + μ(B), not equality — useful but conservative.
- The bound ‖e^{At}‖ ≤ e^{μ(A) t} is sharp only for normal A.

## Related Concepts

- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/contractivity]]
- [[concepts/von-neumann-theorem]]
- [[concepts/kreiss-matrix-theorem]]
- [[concepts/singular-perturbation-problem]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
