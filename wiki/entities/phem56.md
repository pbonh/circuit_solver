---
title: PHEM56
type: entity
id: entity-phem56
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
---

## Overview

PHEM56 is A. Murua's (1995) half-explicit Runge-Kutta code for index-2 constrained mechanical systems (Sect. VII.6 of Hairer-Wanner Vol. II). It is the projected half-explicit RK family whose order-6 stage coefficients are constructed in Murua (1995), built on top of Brasey's HEM5 lineage and Arnold's improved HEX5 variant.

## Characteristics

- Half-explicit RK: explicit on the differential variables, implicit (small algebraic system) only on the Lagrange multipliers / constraint.
- Order 5(6) — fifth-order method with sixth-order embedded estimate.
- Successor of HEM5 (Brasey 1994) and HEX5 (Arnold 1995); incorporates Murua's coefficient sets for the constrained Hamiltonian system (eq. 6.16a-d).
- On the Sect. VII.7 mechanical-system benchmark (Fig. 7.3), PHEM56 is slightly less efficient than DOPRI5_VEL on problems where `g_qq(q)(v,v)` is cheap, but the literature reports it superior when that evaluation is expensive.
- Like DOPRI5 and MEXX, it does not handle stiff cases — for very stringent tolerances the codes are forced to follow the highly oscillatory exact solution.

## Common Strategies

- The explicit-DAE workhorse for non-stiff constrained mechanical systems (multibody / robotics) at engineering tolerances.

## Related Entities

- A. Murua — author.
- V. Brasey — author of HEM5 (predecessor).
- M. Arnold — author of HEX5.
- [[entities/dopri5]] — explicit comparator with velocity projection.

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
