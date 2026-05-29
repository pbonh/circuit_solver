---
title: 'Computer Methods for Circuit Analysis and Design — Appendix D: Program for
  Network Analysis'
type: source
id: source-computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis
kind: derived-summary
tags:
- software
- foundational
- analog
- sensitivity
- well-established
- simulator
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/24-appendix-d-program-for-network-analysis.txt
---

## Key Points

- Appendix D is a FORTRAN program implementing many of the methods of the textbook for linear analog and digital network analysis. Intended for instructional purposes.
- Three formulation methods: MNT1 (modified-nodal of Section 4.4), MNT2 (two-graph modified-nodal of Section 4.8), and a digital network formulation. For analog: T = G + sC; for digital: T = G + z^{-1} C.
- Analysis types:
  - Frequency-domain: AC sweep with sensitivities (real/imaginary or dB/phase).
  - Pole/zero: METHOD=1 uses polynomial interpolation + root-finding (Chapter 7); METHOD=2 uses QZ algorithm. Q, omega_0, and their sensitivities are computed.
  - Symbolic analysis: from Chapters 7 (one-variable) and 8 (multi-variable, in delta or in element values).
  - Time-domain: numerical Laplace transform inversion (Chapter 10) with stepping. METHOD=0 uses Pade (M=2, N=0) order-2 integration (small step, one factorization per step); METHOD=1 uses high-order Pade (5 factorizations per step) equivalent to order-18 integration (large steps).
- Restrictions: ≤ 40 elements, ≤ 10 sources, ≤ 10 outputs, system matrix size ≤ 25, ≤ 5 symbolic elements, ≤ 10 device models, no mixed analog/digital networks.
- Built-in linear models for bipolar and FET transistors and op-amps (parameterized by up to 8 parameters each).
- Input via card-based syntax: TITLE, MODEL, NET, END, SENSITIVITY, AC, POLE, SYMB, TIME, NEXT, STOP commands.
- "Primitive" sparse solver with partial pivoting for numerical stability — saves operations but not storage.
- Extensive input error diagnostics. Table-driven formulation is easy to modify and extend.

## Relevant Concepts

- [[concepts/modified-nodal-analysis]] — Already covered (MNT1).
- [[concepts/two-graph-modified-nodal]] — Already covered (MNT2).
- [[concepts/digital-network-analysis]] — Already covered.
- [[concepts/symbolic-function-generation]]
- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/qz-algorithm]]
- [[concepts/sparse-matrix-methods]]
- [[entities/watand]] — Larger, production version of the same concepts.

## Source Metadata

- Source type: book appendix (software listing and documentation)
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix D — Program for Network Analysis
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/24-appendix-d-program-for-network-analysis.txt`
- Authors: Jiri Vlach, Kishore Singhal
