---
title: "RADAU5"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt", "raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt"]
confidence: medium
---

## Overview

RADAU5 is the Hairer-Wanner reference Fortran code (Subroutine RADAU5 in the volume's Appendix) for the 3-stage Radau IIA implicit Runge-Kutta method of order 5. It is A-stable, L-stable, stiffly accurate, and is one of the codes against which all other stiff and DAE codes are benchmarked in the book.

## Characteristics

- 3-stage [[concepts/radau-iia-method]] (order 5), L-stable and stiffly accurate.
- Implementation details given in Sect. IV.8: simplified Newton iteration on the stage equations, eigenvalue transformation that produces one real and one complex block in the linear algebra, predictive PI step-size controller (Sect. IV.8 step size selection).
- Embedded order-4 error estimator (Sect. IV.8, formula at 7.123-style "embedded formula for RADAU5") for local error control.
- Used as the workhorse for index-1, index-2, and index-3 DAE solving via the stiffly-accurate property (Sect. VI.4, VII.4).
- Applicable to the linearly implicit form `M y' = f(t,y)` via the appendix's option for a mass matrix (Sect. VII.4 mechanical system results).

## Common Strategies

- Reference comparator for [[concepts/rosenbrock-method]] codes ([[entities/rodas]], [[entities/rodas5]], [[entities/ros4]]) and [[concepts/extrapolation-method]] codes ([[entities/seulex]], [[entities/sodex]]) in Sect. IV.10's work-precision diagrams.
- For mechanical systems with banded Jacobians the linear-algebra separation lets users plug in a special solver (e.g., the "second-order" reduction of beam problems, Sect. IV.10 — without it RADAU5 would be ~3x slower).
- Higher-order generalization [[entities/radaup]] (RADAUP) lets the user switch between s=3, 5, 7 (orders 5, 9, 13); mathematically equivalent to RADAU5 at s=3 but slightly slower due to a more general coding.

## Related Entities

- [[entities/ernst-hairer]], [[entities/gerhard-wanner]] — authors and code distributors.
- [[entities/rodas]], [[entities/rodas5]], [[entities/ros4]], [[entities/sdirk4]], [[entities/seulex]], [[entities/sodex]] — companion stiff codes from Sect. IV.10.
- [[entities/dopri5]] — nonstiff comparator.
- [[entities/lsode]], [[entities/dassl]] — multistep/DAE comparators.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
