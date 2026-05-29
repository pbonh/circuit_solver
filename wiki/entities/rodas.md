---
title: RODAS
type: entity
id: entity-rodas
tags:
- ode
- numerical-integration
- foundational
- dae
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
---

## Overview

RODAS is Hairer-Wanner's stiffly-accurate [[concepts/rosenbrock-method]] code of order 4 with an embedded 3rd-order error estimator. Its coefficients satisfy the extra order conditions needed to extend Rosenbrock methods to index-1 [[concepts/differential-algebraic-equation]] systems (Sect. VI.4) at the cost of "a little more work per step" than [[entities/ros4]].

## Characteristics

- 4th-order Rosenbrock with 3rd-order embedded estimate; linear algebra completely separated from the integrator core, letting users plug in special block solvers (Sect. IV.10's BEAM/PLATE results).
- Stiffly accurate — the final stage equals the new solution — which is what makes it usable for index-1 DAEs without auxiliary steps.
- Best one-step code at low tolerances (Tol = 10⁻² to 10⁻⁵) across the standard test set (VDPOL, ROBER, OREGO) per Fig. IV.10.8.
- Sensitive to round-off at stringent tolerances (Sect. IV.10 — drops to order ~1 on the carbon-circuit problem where the linear system is ill-conditioned).
- Extension [[entities/rodas5]] (Di Marzo 1992) raises the order to 5.

## Common Strategies

- Default choice for industrial-grade stiff and index-1 DAE problems at engineering tolerances.
- Used together with the partitioned variant for problems where part of the state is non-stiff.

## Related Entities

- [[entities/ros4]] — order-4 Rosenbrock without the DAE order conditions.
- [[entities/rodas5]] — order-5 successor by Di Marzo.
- [[entities/radau5]], [[entities/seulex]], [[entities/sdirk4]] — companion stiff one-step codes.
- [[entities/dassl]], [[entities/lsode]] — multistep counterparts.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
