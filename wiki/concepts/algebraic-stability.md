---
title: "Algebraic Stability"
type: concept
tags: [ode, numerical-integration, stiff, stability, nonlinear, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A Runge–Kutta method with tableau (A, b, c) is algebraically stable (Burrage–Butcher 1979, Crouzeix 1979) if (i) b_i ≥ 0 for every weight, and (ii) the matrix M = B A + A^T B − b b^T (with B = diag(b_i)) is non-negative definite. This is a sufficient algebraic condition for [[concepts/b-stability]] and, for S-irreducible methods, equivalent to it (Hundsdorfer–Spijker 1981).

## How It Works

The test (b_i ≥ 0, M ⪰ 0) is checked directly from the tableau. Multiplying the simplified residual equation by B and computing the quadratic form (Z, B Z) on stage differences yields the key inequality (Z, B Z) − ‖∑ b_i Z_i‖^2 = (Z, M Z) ≥ 0 — exactly the discrete contractivity bound. The condition is invariant under irreducible rotations of the tableau, so it characterises the *method* rather than its parametrisation. Gauss, Radau IA, Radau IIA, and Lobatto IIIC are algebraically stable; Lobatto IIIA and IIIB fail because they have b_s = 0 or sign-indefinite M.

## Key Parameters

- Diagonal matrix B = diag(b_i).
- Symmetric matrix M = BA + A^T B − bb^T.
- Weight non-negativity check (cheap).
- Definiteness check (eigenvalues of M, or Cholesky test).

## When To Use

- Verifying B-stability of a new method.
- Constructing algebraically stable Runge–Kutta families via the [[concepts/w-transformation]].
- Establishing the algebraic-stability prerequisite for B-convergence theorems (e.g., Frank–Schneid–Ueberhuber Theorem 15.3).

## Risks & Pitfalls

- The matrix M can be subtly indefinite even when b_i ≥ 0; check eigenvalues numerically as well as analytically.
- Algebraic stability does not preserve under reducible decompositions; analyse the S-irreducible form first.
- The corresponding criterion for general linear methods (Burrage–Butcher 1979, §V.9) requires a *block* positive-definite condition, not a scalar one.

## Related Concepts

- [[concepts/b-stability]]
- [[concepts/an-stability]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/runge-kutta-method]]
- [[concepts/radau-iia-method]]
- [[concepts/gauss-method]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/w-transformation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
