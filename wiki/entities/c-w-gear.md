---
title: C. W. Gear
type: entity
id: entity-c-w-gear
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
---

## Overview

Charles William Gear is the author of the 1971 monograph "Numerical Initial Value Problems in Ordinary Differential Equations" (Prentice-Hall), which Hairer-Wanner cite as the moment when BDF-based codes became "the most prominent and most widely used for all stiff computations" (Sect. V opening). The Gear-DIFSUB code is the direct ancestor of [[entities/lsode]], VODE, DEBDF, and ultimately DASSL.

## Characteristics

- Gear (1971) — codified [[concepts/gear-bdf]] for stiff initial-value problems; the same year, "Simultaneous numerical solution of differential-algebraic equations" — the first treatment of [[concepts/differential-algebraic-equation]]s.
- Gear (1982) — automatic detection and treatment of oscillatory and/or stiff ODEs.
- Gear (1988) — differential-algebraic equation index transformations; foundational for index-reduction work in Sect. VII.2.
- Gear (1990) — differential-algebraic equations, indices, and integral algebraic equations.
- Campbell & Gear (1995) — the index of general nonlinear DAEs (with [[concepts/index-of-a-dae]] formalism).
- Gear, Hsu & Petzold (1981) / Gear & Petzold (1983, 1984) — joint papers with [[entities/linda-petzold]] that produced the DAE / matrix-pencil theory consumed by [[entities/dassl]].
- Gear & Saad (1983) — iterative solution of linear equations in ODE codes.
- Gear, Gupta & Leimkuhler (1985) — automatic integration of Euler-Lagrange equations (constrained mechanical systems).

## Common Strategies

- Variable-order BDF with Nordsieck representation (used in DIFSUB, [[entities/lsode]], VODE).
- Differentiation index reduction for higher-index DAEs (Sect. VII.2).

## Related Entities

- [[entities/linda-petzold]] — long-term co-author and DASSL author.
- A.C. Hindmarsh — author of LSODE which descends from DIFSUB.
- [[entities/germund-dahlquist]] — provided the underlying stability theory.

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
