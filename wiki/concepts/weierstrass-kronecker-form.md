---
title: Weierstrass–Kronecker Form
type: claim
id: claim-weierstrass-kronecker-form
tags:
- dae
- linear-algebra
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

The Weierstrass–Kronecker canonical form (Theorem 1.1 in Hairer–Wanner Chapter VII) states: for any regular matrix pencil (A, B) with det(A + λ B) ≢ 0, there exist invertible matrices P, Q such that P A Q = diag(C, I) and P B Q = diag(I, N), where C is in (real) Jordan form and N is block-nilpotent.

## How It Works

The decomposition splits the linear DAE B u' + A u = d into two decoupled parts: a regular ODE part u_1' + C u_1 = P_1 d (size = rank of finite Jordan blocks) and a nilpotent algebraic part N u_2' + u_2 = P_2 d (size = nilpotent block size). Iterating u_2 = −N u_2' + P_2 d and using N^k = 0 gives u_2 = −P_2 d + N P_2 d' − N^2 P_2 d'' + … ± N^{k−1} P_2 d^{(k−1)}: the algebraic part is a *finite* derivative-array combination of d. The index k of N is the [[concepts/index-of-nilpotency]] = differentiation index of the linear pencil.

## Key Parameters

- Pencil (A, B), regular.
- Transformation matrices P, Q.
- Jordan structure of C, nilpotent structure of N.

## When To Use

- Linear constant-coefficient DAE analysis.
- Descriptor-system theory in control engineering.
- Local linearisation of nonlinear DAEs (matrix pencil at a point).

## Risks & Pitfalls

- Existence requires regularity of the pencil; *singular* pencils (det ≡ 0) require Kronecker's general canonical form with rectangular blocks.
- Numerical computation of P, Q can be ill-conditioned when blocks of N are large.
- The decomposition is global but not unique; multiple Jordan-style bases exist.

## Related Concepts

- [[concepts/matrix-pencil]]
- [[concepts/index-of-nilpotency]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/differentiation-index]]
- [[concepts/index-of-a-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
