---
title: "DOPRI5"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt", "raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt"]
confidence: medium
---

## Overview

DOPRI5 is the Dormand-Prince 5(4) explicit Runge-Kutta code from Hairer-Norsett-Wanner Volume I. In Volume II it is used as the canonical nonstiff comparator that motivates stiff methods: it illustrates step-size restriction by stability (rather than precision) in the opening Robertson reaction example (Sect. IV.1, Fig. 1.3) where its 209 / 205 steps at Rtol = 1e-2 / 1e-3 contrast with RADAU5's 13.

## Characteristics

- Explicit Runge-Kutta of order 5 (embedded order 4 error estimate) — see [[concepts/explicit-runge-kutta]].
- Companion higher-order DOP853 (8th order) is used the same way in Sect. IV.1.
- Stops on stiff problems with `Iidid = -4` ("the problem appears to be stiff") via automatic stiffness detection (Sect. IV.1).
- DOPRI5_VEL is a velocity-projection variant used in the constrained-mechanical-system comparisons of Sect. VII.7.

## Common Strategies

- Reference workpoint for showing how stiff codes ([[entities/radau5]], [[entities/rodas]], [[entities/seulex]]) save orders of magnitude in step count.
- On mildly stiff or large but cheap problems (BEAM, CUSP) it competes well with the multistep code [[entities/lsode]] in LADAMS mode.

## Related Entities

- DOP853 — higher-order sibling.
- [[entities/radau5]], [[entities/rodas]], [[entities/seulex]] — stiff comparators.
- [[entities/phem56]] — explicit comparator for constrained mechanical systems.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
