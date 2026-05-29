---
title: Linda R. Petzold
type: entity
id: entities/linda-petzold
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
---

## Overview

Linda R. Petzold is the American numerical analyst whose DASSL code (Petzold 1982) is the standard against which all later DAE codes are benchmarked in Hairer-Wanner Vol. II. Her own bibliography in the book runs to seven entries spanning DAE solver development and the joint Gear-Petzold theory of differential/algebraic systems and matrix pencils.

## Characteristics

- Petzold (1982) — original [[entities/dassl]] paper, described in detail in Brenan, Campbell & Petzold (1989) — "Numerical Solution of Initial-Value Problems in Differential-Algebraic Equations" (the standard monograph).
- Brenan & Petzold (1989) — "The numerical solution of higher index differential-algebraic equations."
- Gear, Hsu & Petzold (1981); Gear & Petzold (1983, 1984) — joint theory of differential/algebraic systems and matrix pencils; underpins the index-1 theory in Sect. VII.3.
- Ascher & Petzold (1991) — projected implicit Runge-Kutta methods for differential-algebraic systems (cited in Sect. VII.6 for the projection family that includes [[concepts/projected-runge-kutta]]).

## Common Strategies

- Variable-order BDF for fully-implicit DAEs `F(t,y,y') = 0`.
- Projection methods for higher-index DAEs.

## Related Entities

- [[entities/c-w-gear]] — long-term co-author.
- K.E. Brenan, S.L. Campbell — co-authors of the 1989 monograph.
- U.M. Ascher — joint projected-RK work.

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
