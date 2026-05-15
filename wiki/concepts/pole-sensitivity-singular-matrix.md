---
title: "Pole Sensitivity via Singular-Matrix LU"
type: concept
tags: [sensitivity, analog, ac, well-established, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt"]
confidence: medium
---

## Definition

At a pole p of a network function, the system matrix T(p) is singular and the standard direct/adjoint solves break down. Vlach & Singhal's technique exploits the LU factorization at the singular point: l_nn = 0 by construction, and the null vectors of L and U^T provide the right- and left-eigenvectors needed for pole sensitivity computation.

## How It Works

Factor T(p) = LU with pivoting. At the pole, l_nn = 0. Define X and X^a as solutions of UX = e_n and L^T X^a = l_nn e_n (right-hand side is zero). Choose x_n^a = 1; recover x_{n-1}^a, ..., x_1^a by back substitution. Then:

d l_nn / d h = (X^a)^T (dT/dh) X (Eq. 6.5.19),
d p / d h = -(d l_nn / d h) / (d l_nn / d s) (Eq. 6.5.20).

The (d l_nn / d s) denominator equals (X^a)^T C X. Permutation matrices Pi_r, Pi_c handle pivoting: Pi_r T Pi_c = LU; then Pi_c X and Pi_r^T X^a are the actual null vectors.

## Key Parameters

- Multiplicity of the pole (formula assumes simple).
- Pivoting strategy used during LU factorization.
- Numerical sensitivity of locating the pole (necessary to find p with high accuracy first).

## When To Use

- Computing pole sensitivities to element values without using polynomial root-finding.
- High-order networks where symbolic pole expressions are infeasible.
- Stability-sensitivity analyses (movement of poles with respect to design parameters).

## Risks & Pitfalls

- The pole must be located accurately first; the factorization at the wrong frequency does not give a singular L.
- Multiple poles require generalized eigenvector techniques not covered here.
- Numerical conditioning near the pole is poor; high-precision arithmetic may be needed.

## Related Concepts

- [[concepts/pole-zero-sensitivity]]
- [[concepts/transpose-system-method]]
- [[concepts/lu-decomposition]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
