---
title: J. R. (Jeff) Cash
type: entity
id: entity-jeff-cash
tags:
- ode
- numerical-integration
- foundational
- bdf
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
---

## Overview

J.R. (Jeff) Cash is a British numerical analyst whose programme of "extended" backward-differentiation formulas (MEBDF and SECDER) extends the BDF order ceiling beyond Dahlquist's barrier and is the multistep counterpart that closes Sect. V.5 of Hairer-Wanner Vol. II.

## Characteristics

- Cash (1976) — semi-implicit Runge-Kutta procedures with error estimates for stiff systems.
- Cash (1979a) — diagonally implicit Runge-Kutta formulae with error estimates.
- Cash (1979b) — stable recursions, with applications to the numerical solution of stiff systems.
- Cash (1980) — "On the integration of stiff systems of O.D.E.s using extended backward differentiation formulae" — original [[concepts/extended-bdf-method]] paper.
- Cash (1981) — second derivative extended BDF for the numerical [solution of stiff ODEs] (referenced for [[concepts/sdbdf-method]]).
- Cash (1983) — integration of stiff initial value problems in ODEs using modified extended BDF.
- Cash & Considine (1992) — an MEBDF code for stiff initial value problems (ACM TOMS) — the published code that Sect. V.5 compares against [[entities/lsode]] and VODE.

## Common Strategies

- Extended BDF (EBDF) and modified extended BDF (MEBDF) — adds a "super-future" point to widen the stability region.
- Cash's algorithm (subject index 268) — stability/contractivity construction.

## Related Entities

- S. Considine — MEBDF code co-author.
- A.C. Hindmarsh — [[entities/lsode]] benchmark counterparty.

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
