---
title: RODAS5
type: entity
id: entity-rodas5
tags:
- ode
- numerical-integration
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
---

## Overview

RODAS5 is G.A. Di Marzo's (1992) order-5 extension of [[entities/rodas]], with coefficients constructed to satisfy the index-1 DAE order conditions at order 5. The book cites it as "RODAS5(4), methodes de Rosenbrock d'ordre 5(4) adaptees aux..." (bibliography, Di Marzo 1992) and uses it for high-precision Rosenbrock comparisons.

## Characteristics

- 5th-order [[concepts/rosenbrock-method]] with embedded 4th-order estimator.
- Stiffly accurate; same separated-linear-algebra interface as RODAS.
- In Sect. IV.10's Fig. 10.12 comparison of Rosenbrock methods, RODAS5 (order 5) and RODAS (order 4) often give similar wall-clock performance despite RODAS5's larger stage count, because higher order pays off at the same tolerance.

## Common Strategies

- Used when RODAS's order is the bottleneck — i.e., tolerances where the order-4 work curve crosses RODAS5's.

## Related Entities

- [[entities/rodas]] — order-4 predecessor.
- [[entities/ros4]] — bare order-4 Rosenbrock without DAE conditions.
- G.A. Di Marzo — constructor of the coefficient set.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
