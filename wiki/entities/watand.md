---
title: WATAND (Waterloo Analysis and Design)
type: entity
id: entities/watand
tags:
- software
- cad
- simulator
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/01-preface.txt
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt
---

## Overview

WATAND (Waterloo Analysis and Design) is an interactive circuit analysis and design package developed at the University of Waterloo. It features advanced graphics capabilities and was used to solve the larger realistic examples illustrated in Vlach and Singhal's *Computer Methods for Circuit Analysis and Design*. Many Waterloo colleagues, past and present, contributed to its development.

## Characteristics

- Interactive use model with advanced graphics output (notable for the early 1980s era).
- Implements the modern CAD techniques described in the book: sparse-matrix formulation, sensitivity analysis, frequency- and time-domain analysis, optimization.
- Used at the University of Waterloo for instruction and research.

## Common Strategies

- Modified-nodal and tableau formulations of network equations.
- Sparse storage and LU factorization for large practical networks.
- Adjoint sensitivity computation as a gradient source for built-in optimization.

## Related Entities

- [[entities/university-of-waterloo]] — Host institution and primary developer of WATAND.
- [[entities/jiri-vlach]] — Contributor.
- [[entities/kishore-singhal]] — Contributor.

## Sources

- [[summaries/computer-methods-circuit-analysis-design-01-preface]]
- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
