---
title: "LSODE"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt", "raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt"]
confidence: medium
---

## Overview

LSODE is Hindmarsh's "Livermore Solver" (1980, 1983), the canonical variable-step, variable-order multistep code. In Hairer-Wanner Vol. II it is the multistep representative in stiff-problem benchmarks (Sect. IV.10 and the entire Sect. V.5 numerical-experiment block).

## Characteristics

- Stiff modes use `MF = 21, 22, 24, 25` and run the Nordsieck representation of fixed step-size [[concepts/gear-bdf]] (covered in book Sects. III.6, III.7).
- Nonstiff mode `MF = 10` is the explicit Adams family, called "LADAMS" in the book's Sect. V.5 plots — included to show how an explicit multistep code performs on large, mildly stiff problems.
- Evolved from C.W. Gear's DIFSUB (1971) lineage; its user interface set the de-facto standard for stiff ODE codes (Sect. V.5).
- Available via `send lsode.f from odepack` (netlib).

## Common Strategies

- Multistep BDF benchmark against one-step stiff codes ([[entities/radau5]], [[entities/rodas]], [[entities/seulex]]) in Sect. IV.10. Generally slightly slower than one-step codes when f-evaluations are cheap; competitive when they are not.
- Drives related codes: [[entities/dassl]]-family (DAE), DEBDF (Shampine & Watts wrapper), [[entities/lsodi]] (implicit form).

## Related Entities

- A.C. Hindmarsh — author.
- [[entities/c-w-gear]] — DIFSUB ancestor.
- [[entities/dassl]] — DAE sibling.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
