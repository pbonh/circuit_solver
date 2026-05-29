---
title: EULSIM
type: entity
id: entity-eulsim
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

EULSIM (Deuflhard 1985) is the predecessor to [[entities/seulex]], implementing the same stiff linearly implicit Euler extrapolation method but with a different implementation structure.

## Characteristics

- Predecessor of [[entities/seulex]] (Hairer-Wanner).
- Implements the Stiff linearly implicit Euler extrapolation method.
- Uses the same numerical method as SEULEX but with an earlier codebase.

## Related Entities

- [[entities/seulex]] — successor code with refined step-size and order selection.
- [[entities/sodex]] — related extrapolation code based on the Bader-Deuflhard midpoint rule.
