---
title: "ODEX"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-21
updated: 2026-05-21
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-iv-stiff-problems-one-step-methods.txt"]
confidence: low
---

## Overview

ODEX is the Hairer-Wanner non-stiff extrapolation code (Sect. II.9 of Hairer-Wanner Vol. II). It implements the explicit midpoint-rule extrapolation method with step-size and order selection logic that is reused (with minor changes) by the stiff counterpart [[entities/sodex]].

## Characteristics

- Non-stiff extrapolation code based on the explicit midpoint rule.
- Step-size and order selection logic shared with [[entities/sodex]].
- Part of the Hairer-Wanner extrapolation solver family.

## Related Entities

- [[entities/sodex]] — stiff counterpart using the Bader-Deuflhard linearly implicit midpoint rule.
- [[entities/seulex]] — stiff extrapolation code based on linearly implicit Euler.
