---
title: Alexander Ostermann
type: entity
id: entities/alexander-ostermann
tags:
- ode
- numerical-integration
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/01-preface.txt
- raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
- raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-vi-singular-perturbation-problems.txt
---

## Overview

Alexander Ostermann is a numerical analyst long associated with the Geneva seminar group. The book's preface thanks him together with Ch. Lubich for reading the whole text and contributing "innumerable corrections and suggestions for improvement". His own published contributions used in the book span Rosenbrock-method construction, dense-output theory for extrapolation, and contractivity analysis for linearly implicit methods on singular-perturbation problems.

## Characteristics

- Hairer & Ostermann (1990) — dense output for extrapolation methods (Theorems 5.7 and 5.8 in Sect. VI.5 of the book), valid for any step-number sequence.
- Kaps & Ostermann (1989, 1990) — Rosenbrock methods using few LU-decompositions and L(α)-stable variable-order Rosenbrock methods; underpins the higher-order Rosenbrock theory used in Sect. IV.7.
- Ostermann (1988) — contractivity and convergence proofs for linearly implicit methods on nonlinear singular-perturbation problems where `hL = O(hε⁻¹)`.
- Lubich & Ostermann (1993) — Runge-Kutta methods for parabolic equations and convolution quadrature (bibliography entry 787).

## Common Strategies

- Order conditions and dense-output construction for extrapolation methods on stiff and DAE problems.
- Contractivity analysis for stiff linearly implicit methods.

## Related Entities

- [[entities/ernst-hairer]] — long-term collaborator.
- [[entities/christian-lubich]] — co-author and Geneva seminar partner.
- P. Kaps — Rosenbrock-method co-author.

## Sources

- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
