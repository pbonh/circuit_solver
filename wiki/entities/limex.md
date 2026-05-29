---
title: LIMEX
type: entity
id: entities/limex
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-vi-singular-perturbation-problems.txt
---

## Overview

LIMEX is Deuflhard-Nowak's [[concepts/extrapolation-method]] code for quasilinear implicit ODEs (Deuflhard & Nowak 1987 in "Large-Scale Scientific Computing", Birkhäuser). In Hairer-Wanner Vol. II the code's basic step (a linearly-implicit Euler step on `M(y) y' = f(y)`) is the worked example in Sect. VI.6 for [[concepts/quasilinear-dae]] (book reference: "which represents the basic step for the code LIMEX described in Deuflhard & Nowak (1987)"). Cited again at p. 448 of the subject index for the quasilinear-DAE chapter.

## Characteristics

- Extrapolation tableau built on the linearly implicit Euler base method for `M(y) y' = f(y)`.
- Regularity of the basic-step linear system is guaranteed by Lemma 6.2 of Sect. VI.6.
- Requires an approximation to Z₀ = Yb' for the Jacobian; the consistent initial values for the first basic steps are computed explicitly.

## Common Strategies

- Used on quasilinear DAEs where a state-dependent mass matrix appears (the chemistry / multibody examples in Sect. VI.6 / VII.6).

## Related Entities

- [[entities/peter-deuflhard]] — author.
- U. Nowak — co-author.
- [[entities/seulex]], [[entities/sodex]] — sibling extrapolation codes for ODEs.

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
