---
title: SDIRK4
type: entity
id: entity-sdirk4
tags:
- ode
- numerical-integration
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
---

## Overview

SDIRK4 is the Hairer-Wanner reference implementation of the [[concepts/sdirk-method]] of order 4 constructed in Sect. IV.6 (formula (6.16) — the 5-stage L-stable SDIRK). It is the diagonal-implicit comparison code in Sect. IV.10's benchmarks. Implementation details are in Sect. IV.8 alongside RADAU5; the section explicitly notes that "if the Runge-Kutta matrix has at least one real eigenvalue" (which singly-diagonal SDIRK satisfies trivially), the implementation is straightforward.

## Characteristics

- 5-stage singly-diagonally-implicit Runge-Kutta method of order 4, L-stable, stiffly accurate (formula (6.16) in Sect. IV.6).
- Continuous (dense) output (Sect. IV.6, "dense output of SDIRK4" index entry).
- Does NOT have an option for the "second order" linear-algebra reduction used by RADAU5/RODAS/SEULEX on BEAM-like problems.

## Common Strategies

- Demonstrates "diagonally implicit only" performance: gives rather disappointing results compared to fully implicit RK and Rosenbrock codes in Sect. IV.10 Fig. 10.11, except on the BEAM problem where its dense output and stiff-decoupling shine.

## Related Entities

- [[entities/radau5]] — fully-implicit RK comparator at the same workpoint.
- [[entities/rodas]], [[entities/ros4]] — Rosenbrock comparators.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
