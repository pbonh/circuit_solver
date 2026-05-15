---
title: "Index of Nilpotency"
type: concept
tags: [dae, linear-algebra, classification, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

For a regular linear constant-coefficient matrix pencil B u' + A u = d with det(A + λ B) ≢ 0, the [[concepts/weierstrass-kronecker-form]] yields P A Q = diag(C, I), P B Q = diag(I, N) with N block-nilpotent. The index of nilpotency is the size of the largest Jordan block of N — equivalently the smallest k such that N^k = 0.

## How It Works

The nilpotent block N produces the algebraic part of the solution; differentiating it k times gives the underlying ODE, so the index of nilpotency equals the [[concepts/differentiation-index]] of the linear pencil. For nonlinear DAEs the analogous local concept involves the *matrix pencil at a point*: ∂F/∂u' + λ ∂F/∂u, and its nilpotency index gives the local linear-pencil approximation to the differentiation index.

## Key Parameters

- Pencil (A, B).
- Jordan-block sizes of N.
- Index k (largest block size).

## When To Use

- Linear constant-coefficient DAE analysis (circuit theory, control-system descriptors).
- Local linearisation of nonlinear DAEs around a solution branch.
- Theoretical bridge between linear pencil theory and the differentiation index.

## Risks & Pitfalls

- Only defined for *regular* pencils (det(A + λ B) ≢ 0); singular pencils need a different theory.
- The Jordan structure is sensitive to perturbations — small generic perturbations regularise the pencil and shift the index.

## Related Concepts

- [[concepts/weierstrass-kronecker-form]]
- [[concepts/matrix-pencil]]
- [[concepts/differentiation-index]]
- [[concepts/index-of-a-dae]]
- [[concepts/perturbation-index]]
- [[concepts/differential-algebraic-equation]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
