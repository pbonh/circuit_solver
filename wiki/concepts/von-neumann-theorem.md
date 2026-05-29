---
title: Von Neumann's Theorem
type: claim
id: concepts/von-neumann-theorem
tags:
- ode
- numerical-integration
- stability
- linear-algebra
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Von Neumann's theorem (1951; see also Riesz–Sz.-Nagy 1955) bounds the spectral norm of a rational function R applied to a contraction A: if A is a Hilbert-space contraction (‖A‖ ≤ 1) and R is rational, bounded by 1 on the closed unit disk, then ‖R(A)‖ ≤ 1. Equivalently, ‖R(A)‖ ≤ sup_{|z| ≤ 1} |R(z)|. In numerical-ODE form, the bound transfers to ‖R(hA)‖ ≤ sup_{z ∈ S} |R(z)| whenever the spectrum and pseudo-spectrum of hA lie inside the [[concepts/stability-region]] S.

## How It Works

The theorem links scalar-stability-function arguments to operator-norm bounds, bypassing the eigenvalue / non-normality gap that ordinary spectral arguments suffer from. In Hairer–Wanner IV.11, it justifies that A-stability of R(z) implies ‖R(hA)‖ ≤ 1 for any matrix A with logarithmic norm μ(A) ≤ 0 — the *correct* operator bound for stiff linear systems, where the spectrum can mislead because A is non-normal.

## Key Parameters

- Norm on the matrix space (Hilbert-space / spectral).
- Logarithmic norm μ(A).
- Sup of |R(z)| on the relevant region.

## When To Use

- Operator-norm stability proofs for IRK / multistep methods on linear systems with non-normal Jacobians.
- Justification for replacing scalar |R(hλ)| arguments with matrix ‖R(hA)‖ bounds.

## Risks & Pitfalls

- Strict spectral norm only; transfers to other norms with constants.
- For Banach-space settings (e.g. PDE method-of-lines on non-Hilbert function spaces) the [[concepts/kreiss-matrix-theorem]] is the sharper tool.

## Related Concepts

- [[concepts/kreiss-matrix-theorem]]
- [[concepts/logarithmic-norm]]
- [[concepts/stability-function]]
- [[concepts/stability-region]]
- [[concepts/a-stability]]
- [[concepts/holomorphic-semigroup]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
