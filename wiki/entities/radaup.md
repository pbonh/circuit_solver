---
title: RADAUP
type: entity
id: entities/radaup
tags:
- ode
- numerical-integration
- foundational
created: 2026-05-21
updated: 2026-05-21
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-iv-stiff-problems-one-step-methods.txt
---

## Overview

RADAUP is the higher-order generalization of [[entities/radau5]] in the Radau IIA family. It lets the user switch between stage counts s = 3, 5, 7, yielding orders 5, 9, and 13 respectively. At s = 3 it is mathematically equivalent to RADAU5 but slightly slower due to more general coding.

## Characteristics

- Generalized Radau IIA code with selectable stage count (s = 3, 5, 7).
- Orders 5, 9, 13 for stiff ODEs and DAEs.
- Mathematically equivalent to [[entities/radau5]] at s = 3.

## Related Entities

- [[entities/radau5]] — fixed s = 3 (order 5) predecessor, faster for that single order.
