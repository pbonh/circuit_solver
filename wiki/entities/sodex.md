---
title: "SODEX"
type: entity
tags: [ode, numerical-integration, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt"]
confidence: medium
---

## Overview

SODEX is the Hairer-Wanner extrapolation code based on the Bader-Deuflhard (1983) linearly implicit mid-point rule (formula (9.16) of Sect. IV.9), mathematically equivalent to METAN1. It uses the step-number sequence (9.22) and reuses (with minor changes) the step-size / order selection logic of the non-stiff code [[entities/odex]]. In SODEX's work-per-unit-step (formula II.9.26) the count A_k is augmented by the ODE dimension to account for the Jacobian evaluation cost.

## Characteristics

- Adaptive-order [[concepts/extrapolation-method]] applied to the linearly implicit mid-point rule.
- Equivalent to METAN1.
- Used together with [[entities/seulex]] to show how the underlying base method (Euler vs. mid-point) shifts the work-precision curves.
- Sect. IV.10's experiments find SODEX competitive with SEULEX but generally slightly behind.

## Common Strategies

- The "mid-point variant" of stiff extrapolation; useful when smoothness of f makes the mid-point rule's higher accuracy per stage pay off.

## Related Entities

- [[entities/seulex]] — Euler-based sibling.
- METAN1 — equivalent earlier code.
- [[entities/peter-deuflhard]] — co-developer of the base method.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
