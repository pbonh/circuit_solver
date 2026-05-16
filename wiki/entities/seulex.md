---
title: "SEULEX"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt"]
confidence: medium
---

## Overview

SEULEX is the Hairer-Wanner extrapolation code implementing the Stiff linearly implicit EULer EXtrapolation method (formula (9.32) of Sect. IV.9). It uses the step-number sequence {2, 3, 4, 5, 6, 7, ...} by default (other sequences selectable). The step-size and order selection mirror those of [[entities/sodex]]. The earlier code [[entities/eulsim]]/EULSIM (Deuflhard 1985) is the predecessor with the same numerical method but a different implementation.

## Characteristics

- Adaptive-order, adaptive-step [[concepts/extrapolation-method]] applied to the linearly implicit Euler method.
- A(α)-stable (book Sect. IV.9 stability analysis).
- Best one-step code for stringent tolerances on the standard stiff battery (Sect. IV.10 Fig. 10.8): superior to Rosenbrock codes at Tol ≤ 10⁻⁶ on VDPOL/ROBER/OREGO.
- "SEULEX has problems with round-off errors at high precision" on the carbon-circuit problem (Sect. IV.10) — the linear-system formulation is round-off-sensitive.

## Common Strategies

- Default high-accuracy stiff code in the Hairer-Wanner benchmark suite alongside [[entities/radau5]] (which dominates at slightly different workpoints).
- Used together with [[entities/sodex]] (mid-point rule extrapolation) to show the linear-Euler vs. mid-point trade-off.

## Related Entities

- [[entities/sodex]] — linearly implicit mid-point rule extrapolation; same step-size logic.
- EULSIM — predecessor implementation by Deuflhard.
- [[entities/peter-deuflhard]] — originator of the extrapolation lineage.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
