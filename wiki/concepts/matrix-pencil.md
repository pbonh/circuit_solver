---
title: Matrix Pencil
type: claim
id: claim-matrix-pencil
tags:
- linear-algebra
- dae
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

A matrix pencil is a one-parameter linear family of matrices λ ↦ A + λ B with A, B ∈ ℝ^{n × n}. It is *regular* if det(A + λ B) ≢ 0 as a polynomial in λ; otherwise *singular*. Generalised eigenvalues are the roots of det(A + λ B) = 0 (including ∞ when B is singular).

## How It Works

Pencils arise from linear DAEs B u' + A u = d as the algebraic object whose Weierstrass–Kronecker decomposition (Theorem VII.1.1) gives the underlying ODE / algebraic split. Numerically, the QZ algorithm computes the generalised Schur decomposition of (A, B) — the canonical pencil-aware analogue of the QR algorithm — yielding generalised eigenvalues and orthogonal transformations to upper-triangular form. The pencil determines the [[concepts/index-of-nilpotency]], the structure of constraints, and the perturbation sensitivity of the system.

## Key Parameters

- Matrices A, B.
- Regularity: det(A + λ B) ≢ 0.
- Generalised eigenvalues (finite and at ∞).
- Index of nilpotency.

## When To Use

- Linear DAE analysis (descriptor systems, modified nodal analysis).
- Linearisation of nonlinear DAEs around solution branches.
- Generalised eigenvalue problems (modal analysis with mass + stiffness matrices).

## Risks & Pitfalls

- Singular pencils require Kronecker's rectangular-block canonical form, not Weierstrass.
- Computing generalised eigenvalues via λ = α/β where (α, β) are the QZ outputs avoids over/underflow when β is tiny.

## Related Concepts

- [[concepts/weierstrass-kronecker-form]]
- [[concepts/index-of-nilpotency]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/differentiation-index]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
