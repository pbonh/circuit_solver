---
title: A0-Stability
type: claim
id: concepts/ao-stability
tags:
- ode
- numerical-integration
- stiff
- stability
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

Cryer's (1973) A0-stability requires the [[concepts/stability-region]] of a method to contain the entire negative real axis (−∞, 0). It is the weakest classical stability requirement of the A-family: weaker than A(α)-stability (which demands a full sector) and far weaker than A-stability (which demands the whole left half-plane).

## How It Works

A method tested on y' = λy with λ < 0 produces y_{n+1} = R(hλ) y_n; A0-stability is |R(x)| ≤ 1 for all x ≤ 0. For BDF formulas, A0-stability holds for all k ≤ 6. The motivation is that purely real negative spectra (e.g. parabolic PDE method-of-lines discretisations with self-adjoint elliptic operators) need only this minimal guarantee. Cryer used the term to classify multistep methods that are practically usable on real-spectrum stiff problems even when they fail A(α)-stability for some α > 0.

## Key Parameters

- Stability function R(z) evaluated on the negative real axis.
- Method order p and whether the principal root tends to zero as z → −∞.

## When To Use

- Diffusion-dominated or self-adjoint stiff problems where Jacobian eigenvalues are real and negative.
- Classification of multistep methods whose stability is too restrictive for general stiff use but adequate on real-spectrum systems.

## Risks & Pitfalls

- A0-stability gives no protection against imaginary-axis eigenvalues; oscillatory modes go unstable.
- Not enough on its own for typical chemical-kinetics stiff problems whose Jacobians have complex eigenvalues.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/a-alpha-stability]]
- [[concepts/stability-region]]
- [[concepts/dahlquist-test-equation]]
- [[concepts/stiff-stability]]
- [[concepts/gear-bdf]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
