---
title: DASSL
type: entity
id: entities/dassl
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
---

## Overview

DASSL is Linda Petzold's BDF-based code for implicit/index-1 differential-algebraic systems (Petzold 1982; described in detail in Brenan-Campbell-Petzold 1989). In Hairer-Wanner Vol. II it is introduced as "the most widely used code" for DAEs (Sect. VII.3) and serves as the multistep DAE reference in the work-precision diagrams of Sect. VII.4 and VII.7.

## Characteristics

- Realization of the [[concepts/gear-bdf]] multistep family for fully-implicit DAEs `F(t, y, y') = 0`.
- Variable-order, variable-step BDF (orders 1-5) with Newton iteration on each step.
- Originally designed for index-1 problems; index-2/index-3 systems require user index reduction (Sect. VII.5 / VII.6 of the book illustrate the failure modes when this is not done).
- Companion codes mentioned alongside: [[entities/lsode]]'s sister [[entities/lsodi]] (Hindmarsh 1980), SPRINT (Berzins & Furzeland 1985).

## Common Strategies

- Used as the multistep BDF DAE benchmark against Runge-Kutta-based DAE solvers ([[entities/radau5]], [[entities/rodas]], [[entities/phem56]]) in Sect. VII.7's constrained-mechanical-system experiments (Fig. 7.3).
- For index-1 semi-explicit form, DASSL applies BDF directly to the algebraic side without distinction (Sect. VII.3, eq. 3.1f).

## Related Entities

- [[entities/linda-petzold]] — author.
- [[entities/c-w-gear]] — originator of the BDF/DIFSUB lineage DASSL extends.
- [[entities/lsode]], [[entities/lsodi]] — sister Livermore solvers.

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
