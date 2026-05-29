---
title: Peter Deuflhard
type: entity
id: entities/peter-deuflhard
tags:
- ode
- numerical-integration
- foundational
- extrapolation
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
- raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-vi-singular-perturbation-problems.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
---

## Overview

Peter Deuflhard is a German numerical analyst (FU Berlin / Zuse-Institut Berlin) whose extrapolation-method programme underlies the entire [[concepts/extrapolation-method]] section IV.9 of Hairer-Wanner Vol. II. His preface acknowledgements appear in both editions, and his bibliography list contains the founding papers for codes [[entities/seulex]], [[entities/sodex]], and [[entities/limex]].

## Characteristics

- Bader & Deuflhard (1983) — "A semi-implicit mid-point rule for stiff systems of ordinary..." — the Bader-Deuflhard method that becomes [[entities/sodex]]/METAN1.
- Deuflhard (1983) — order and stepsize control in extrapolation methods.
- Deuflhard (1985) — extrapolation methods for ordinary differential equations; the paper that introduced EULSIM, predecessor of [[entities/seulex]].
- Deuflhard, Hairer & Zugck (1987) — one-step and extrapolation methods for differential-algebraic equations (Sect. VI.5).
- Deuflhard & Nowak (1987) — extrapolation integrators for quasilinear implicit ODEs; the construction underlying [[entities/limex]].

## Common Strategies

- Adaptive-order, adaptive-step extrapolation tableaux for stiff and DAE problems.
- "Order Window" of Deuflhard — selects the tableau column that minimises work per accepted unit of accuracy.

## Related Entities

- G. Bader — Bader-Deuflhard mid-point rule co-author.
- U. Nowak — LIMEX co-author.
- B. Engquist — co-editor of the 1987 Birkhäuser proceedings where LIMEX first appeared.
- [[entities/ernst-hairer]] — DAE extrapolation co-author.

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
