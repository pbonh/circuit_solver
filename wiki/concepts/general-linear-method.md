---
title: General Linear Method
type: claim
id: claim-general-linear-method
tags:
- ode
- numerical-integration
- multistep
- runge-kutta
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

Butcher's (1966) general linear method (GLM) is a unifying framework for Runge–Kutta and linear multistep integrators. A GLM with r inputs / outputs (U vector of size r) and s internal stages (V vector of size s) advances via U_{n+1} = A U_n + h B̄ f(V_n), V_n = Ā U_n + h B f(V_n), where (A, Ā, B, B̄) are constant matrices. RK methods are GLMs with r = 1; LMS methods are GLMs with s = 1.

## How It Works

The framework lets one analyse stability via the matrix-valued stability function S(μ) = Ā + μ B̄ (I − μ B)^{−1} A, classify order conditions on a richer tree set, and prove general barrier theorems. The [[concepts/daniel-moore-conjecture]] applies in the form: an A-stable GLM with s poles representing numerical work has order ≤ 2s. Algebraic stability ([[concepts/algebraic-stability]]) extends to a block-positive-definite condition on a Burrage–Butcher matrix (1979) generalising the s × s RK case. GLMs include hybrid methods (Butcher–Cash type), [[concepts/multistep-collocation]], and second-derivative methods as special cases.

## Key Parameters

- Number of external inputs r (history depth analogue).
- Number of internal stages s.
- Four matrices (A, Ā, B, B̄).
- Order p and stability properties of S(μ).

## When To Use

- Theoretical unification of LMS and RK theory.
- Constructing new methods that mix history and stage information.
- Proving stability / order barrier theorems for the whole class.

## Risks & Pitfalls

- Order conditions become very combinatorial; practical implementations are rare.
- Notation is heavy; consult Butcher's 2008 book for the modern presentation.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/linear-multistep-methods]]
- [[concepts/multistep-collocation]]
- [[concepts/algebraic-stability]]
- [[concepts/daniel-moore-conjecture]]
- [[concepts/g-stability]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
