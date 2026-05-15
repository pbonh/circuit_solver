---
title: "Partial Fraction Expansion"
type: concept
tags: [foundational, math, well-established, transient]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

Partial fraction expansion decomposes a proper rational function F(s) = N(s)/D(s) into a sum of simple terms K_i/(s - p_i)^{m_i}, one for each pole p_i of multiplicity m_i. It is the workhorse for inverse Laplace transformation of network functions.

## How It Works

Given F(s) with simple poles p_i and residues K_i:
- F(s) = sum K_i / (s - p_i)
- Inverse Laplace transform: f(t) = sum K_i e^{p_i t}.

For a multiple pole of order m:
- f(t) = K t^{m-1} / (m - 1)! * e^{p t}.

Complex-conjugate pole pairs (p = c + j d, residue K = A + j B) combine to give:
- 2 e^{c t} (A cos d t - B sin d t).

Appendix B of Vlach and Singhal derives the residue formulas; Problem P.1.10 in the chapter gives many practice decompositions.

## Key Parameters

- Number and multiplicity of poles.
- Residues K_i (for simple poles, K_i = (s - p_i) F(s) |_{s = p_i}).
- For multiple poles, derivatives of (s - p_i)^m F(s) at s = p_i.

## When To Use

- Analytical time-domain inversion of small Laplace-domain expressions.
- Building intuition for transient behavior.
- Reducing higher-order systems to a sum of first-order responses.

## Risks & Pitfalls

- Repeated or nearly-repeated poles produce ill-conditioned residue calculations.
- Numerical PFE on polynomials of order > ~10 is unreliable; numerical Laplace transform inversion (Chapter 10) avoids this.
- Improper functions (numerator degree >= denominator degree) require polynomial division first.

## Related Concepts

- [[concepts/laplace-transform]]
- [[concepts/poles-and-zeros]]
- [[concepts/numerical-laplace-transform-inversion]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
- [[summaries/computer-methods-circuit-analysis-design-22-appendix-b-partial-fraction-decomposition-of-rational-functions]]
