---
title: 'Computer Methods for Circuit Analysis and Design — Appendix C: Special Complex
  Integration of a Rational Function'
type: source
id: summaries/computer-methods-circuit-analysis-design-23-appendix-c-special-complex-integration-of-a-rational-function
kind: publication
tags:
- foundational
- math
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/23-appendix-c-special-complex-integration-of-a-rational-function.txt
---

## Key Points

- Establishes the lemma needed in Chapter 10's numerical Laplace transform inversion: the integral of a rational function V(s) along an infinite semicircle in the complex s-plane is zero whenever V(s) has at least two more finite poles than zeros (M >= N + 2).
- Proof: parameterize s = R e^{j phi}; for large R the highest powers dominate. The integrand magnitude scales as R^{N - M}; integration along the semicircle gives a factor R, requiring M - N - 1 >= 1, i.e., M >= N + 2 for the contribution to vanish.
- Consequence: when this condition holds, the Bromwich inversion integral equals 2 pi j times the sum of residues at all finite poles in the left half-plane, computable by closed-contour residue calculus.
- This is the theoretical basis for the Pade-based numerical inversion technique of Vlach (1969) used in Chapter 10.

## Relevant Concepts

- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/pade-approximation]]
- [[concepts/laplace-transform]]
- [[concepts/poles-and-zeros]]

## Source Metadata

- Source type: book appendix
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix C — Special Complex Integration of a Rational Function
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/23-appendix-c-special-complex-integration-of-a-rational-function.txt`
- Authors: Jiri Vlach, Kishore Singhal
