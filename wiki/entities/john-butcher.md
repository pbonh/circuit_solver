---
title: John C. Butcher
type: entity
id: entity-john-butcher
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
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
---

## Overview

J.C. (John Charles) Butcher is the New Zealand numerical analyst whose tree-based formalism, implicit-Runge-Kutta theory, and general-linear-method framework underpin most of Chapter IV. The preface explicitly thanks him for "interest in the subject and ... numerous discussions"; his bibliography section in the book has more than a dozen entries from 1964 through 1990.

## Characteristics

- Butcher (1964a) — "Implicit Runge-Kutta processes" — origin of the implicit RK construction used throughout Sect. IV.5.
- Butcher (1964b) — "Integration processes based on Radau quadrature formulas" — the prototype of the [[concepts/radau-iia-method]] family that gives rise to [[entities/radau5]].
- Butcher (1975, 1976, 1977, 1979, 1981, 1982, 1987a-c) — series of papers on the stability and implementation of implicit RK methods, A-stable IRK methods, and singly-implicit (SIRK) methods (Sect. IV.6 — Kuntzmann-Butcher methods index entry).
- Butcher (1987) — "The numerical analysis of ordinary differential equations: Runge-Kutta methods" — the monograph that codified Butcher trees and B-series.
- Butcher (1987b) — equivalence of algebraic stability and AN-stability.
- Butcher (1990) — order, stepsize, and stiffness switching.
- Burrage & Butcher (1979, 1980); Burrage, Butcher & Chipman (1980) — joint papers on implicit RK stability and singly-implicit methods.

## Common Strategies

- Order conditions via Butcher trees (labelled trees, monotonically labelled trees, B-series).
- General linear methods framework — uniformly covers multistep and Runge-Kutta methods.
- Algebraic / AN- / B-stability characterizations used in Sect. IV.12.

## Related Entities

- [[entities/germund-dahlquist]] — counterpart in multistep theory.
- K. Burrage, F.H. Chipman — main co-authors.
- [[entities/syvert-norsett]] — Volume I co-author.

## Sources

- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
