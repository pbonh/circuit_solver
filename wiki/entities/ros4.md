---
title: ROS4
type: entity
id: entity-ros4
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

ROS4 is Hairer-Wanner's [[concepts/rosenbrock-method]] code of order 4 with an embedded 3rd-order error estimator. A method flag lets the user switch among different published 4-stage coefficient sets — the listing in Table 7.2 of Sect. IV.7 (referenced from Sect. IV.10's code-comparison list).

## Characteristics

- 4-stage order-4 Rosenbrock; bare ODE form (the index-1 DAE order conditions are NOT enforced — that is [[entities/rodas]]'s extra requirement).
- Embedded 3rd-order estimate for step-size control.
- Coefficient sets toggle via flag — including the GRK4A/T family (Kaps-Rentrop, cf. book index entry GRK4A 110).
- Linear algebra is separated from the integrator, like RADAU5 and RODAS.

## Common Strategies

- The "what to pick when you don't need DAE compatibility" Rosenbrock baseline against RODAS in Sect. IV.10's experiments.

## Related Entities

- [[entities/rodas]] — DAE-capable sibling at the same order.
- [[entities/rodas5]] — higher-order Rosenbrock.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
