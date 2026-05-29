---
title: 'Computer Methods for Circuit Analysis and Design — Appendix B: Partial Fraction
  Decomposition of Rational Functions'
type: source
id: summaries/computer-methods-circuit-analysis-design-22-appendix-b-partial-fraction-decomposition-of-rational-functions
kind: publication
tags:
- foundational
- math
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/22-appendix-b-partial-fraction-decomposition-of-rational-functions.txt
---

## Key Points

- Decomposes proper rational functions V(s) = N(s)/D(s) into a sum of simple partial fractions for inverse-Laplace-transform table lookup.
- Simple poles: V(s) = sum_i K_i/(s - p_i) with residue K_i = ((s - p_i) V(s))|_{s=p_i}.
- Multiple poles (multiplicity m_i): V(s) gets terms K_{1,i}/(s - p_i) + K_{2,i}/(s - p_i)^2 + ... + K_{m_i,i}/(s - p_i)^{m_i}. Residues from successive differentiation of (s - p_i)^{m_i} V(s) at s = p_i.
- For real coefficients, complex poles appear in conjugate pairs with conjugate residues.

## Relevant Concepts

- [[concepts/partial-fraction-expansion]] — Already covered; this appendix provides the derivations.
- [[concepts/laplace-transform]]
- [[concepts/poles-and-zeros]]

## Source Metadata

- Source type: book appendix
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix B — Partial Fraction Decomposition of Rational Functions
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/22-appendix-b-partial-fraction-decomposition-of-rational-functions.txt`
- Authors: Jiri Vlach, Kishore Singhal
