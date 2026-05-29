---
title: Germund Dahlquist
type: entity
id: entities/germund-dahlquist
tags:
- ode
- numerical-integration
- foundational
- stability
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/01-preface.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
- raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt
---

## Overview

Germund Dahlquist is the Swedish numerical analyst whose multi-decade theoretical programme defines almost the entire stability vocabulary that Hairer-Wanner Vol. II uses. The preface specifically thanks "J. Butcher, G. Dahlquist, and S.P. Nørsett ... for their interest in the subject and for the numerous discussions we had with them which greatly inspired our work." The book carries ten Dahlquist bibliography entries.

## Characteristics

- Dahlquist (1951) — "Fehlerabschätzungen bei Differenzenmethoden..." — foundational error bounds for difference methods.
- Dahlquist (1956) — convergence and stability in the numerical integration of ordinary differential equations: the convergence theorem and Dahlquist's first barrier (order ≤ s+1 for stable s-step methods; subject index 299). See [[concepts/dahlquist-barrier]].
- Dahlquist (1963) — "A special stability problem for linear multistep methods" — introduces [[concepts/a-stability]] and proves the second [[concepts/dahlquist-barrier]] (no A-stable LMM has order > 2; subject index 247, 286, 297, 299).
- Dahlquist (1975, 1978) — error analysis and G-stability theory: "G-stability is equivalent to A-stability" — central to Sect. V.6's one-leg methods.
- Dahlquist & Jeltsch (1979, 1987) — generalized disks of contractivity; reducibility and contractivity of Runge-Kutta methods.
- Dahlquist & Söderlind (1982) — stiff nonlinear differential equations; one-leg vs. multistep equivalences (Dahlquist 1983).
- His test equation `y' = λy` ("Dahlquist's test equation", index 240) is the canonical workpiece for every stability proof in the book.

## Common Strategies

- Linear and nonlinear stability theory for linear multistep methods.
- G-stability framework that links Lyapunov-style energy arguments to A-stability.
- Reducibility / contractivity analysis for Runge-Kutta methods (jointly with Jeltsch).

## Related Entities

- [[entities/john-butcher]] — counterpart in Runge-Kutta theory.
- [[entities/syvert-norsett]] — joint discussant.
- [[entities/olavi-nevanlinna]] — collaborated on contractivity / G-stability.
- R. Jeltsch, G. Söderlind — long-term co-authors.

## Sources

- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
