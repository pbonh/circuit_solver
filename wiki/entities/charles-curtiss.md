---
title: "Charles F. Curtiss"
type: entity
tags: [ode, numerical-integration, foundational, stiff]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt", "raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt", "raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt"]
confidence: medium
---

## Overview

Charles F. Curtiss, jointly with J.O. Hirschfelder, authored the 1952 paper "Integration of stiff equations" (Proc. Nat. Acad. Sci.) — the historically first explicit identification of stiff ODEs and the proposal of BDF methods as the remedy. Hairer-Wanner Vol. II credits this paper as both the origin of the term "stiff" and the foundation for the entire multistep-methods-for-stiff-problems chapter.

## Characteristics

- Curtiss & Hirschfelder (1952) — proposed the practical definition of stiffness still used in Sect. IV.1: "stiff equations are equations where certain implicit methods, in particular BDF, perform better, usually tremendously better, than explicit ones."
- The same paper introduced backward-differentiation-formula (BDF) methods, which Gear (1971) later codified into [[concepts/gear-bdf]].
- Illustrated stiffness with one-dimensional toy problems such as `y' = -50(y - cos x)` (used as eq. (1.1) of Sect. IV.1).

## Common Strategies

- Implicit (BDF) integration as the universal cure for stiffness — the line of work continued by Gear, Hindmarsh, and Petzold.

## Related Entities

- J.O. Hirschfelder — co-author.
- [[entities/c-w-gear]] — codified BDF.
- [[entities/germund-dahlquist]] — provided the stability theory framing BDF.

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
