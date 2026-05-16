---
title: "Laurent Jay"
type: entity
tags: [ode, numerical-integration, foundational, dae, symplectic]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt", "raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt"]
confidence: medium
---

## Overview

Laurent Jay is a numerical analyst (PhD from Université de Genève, Thesis No. 2658, 1994) and another member of the Geneva group, thanked in the second-edition preface for reading the new material. His four 1993-1996 papers furnish the convergence theory for Runge-Kutta methods on index-2 and index-3 DAEs and the symplectic partitioned-RK theory for constrained Hamiltonian systems used in Sect. VII.8.

## Characteristics

- Jay (1993) — "Convergence of a class of Runge-Kutta methods for differential-algebraic systems of index 2" (BIT 33, 137-150) — cited in Sect. VII.4 alongside the HLR89 lecture notes as the reference for stiffly-accurate IRK convergence on index-2 DAEs.
- Jay (1994) — "Runge-Kutta type methods for index three differential-algebraic equations with applications to Hamiltonian systems" — Geneva thesis used as the basis for the Sect. VII.8 superconvergence results and the RATTLE-algorithm extension to general Hamiltonian functions.
- Jay (1995) — "Structure-preserving integrators" (submitted).
- Jay (1996) — "Symplectic partitioned Runge-Kutta methods for constrained Hamiltonian systems" (SIAM J. Numer. Anal., 33, 368-387) — the [[concepts/lobatto-iiia-iiib-pair]] construction adopted in Sect. VII.8.

## Common Strategies

- Order theory for IRK on index-2/index-3 DAEs (the result that constraints do not reduce the order under specific conditions — Sect. VII.8 "very technical and long" proof).
- Symplectic partitioned RK pairs preserving the constraint manifold.

## Related Entities

- [[entities/ernst-hairer]], [[entities/gerhard-wanner]] — Geneva advisors / co-authors.
- [[entities/sebastian-reich]] — symplectic-integration counterpart.

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
