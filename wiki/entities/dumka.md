---
title: DUMKA
type: entity
id: entities/dumka
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

DUMKA is V.I. Lebedev's stabilised-explicit Runge-Kutta code based on optimal-stability-domain [[concepts/chebyshev-method]] polynomials. It incorporates Lebedev's second-order Zolotarev approximation with automatic selection of step size h and the number 3 of stage repetitions in a wide range (Sect. IV.2, eq. 2.49 / Fig. 2.14 / 2.15).

## Characteristics

- Explicit method whose stability region extends along the negative real axis as O(s²) with the number of stages s (Sect. IV.2).
- Internal stages drawn in Fig. 2.15 demonstrate the Chebyshev-recursion construction.
- DUMKA3 (Medovikov; `nucrect@inm.ras.ru`) extends the optimal Lebedev Chebyshev family to third order — used experimentally in Sect. IV.10 Fig. 10.14.
- Sensitive choice of h: at h = 0.48865 on problem (1.6'), DUMKA itself is stable; raw Lebedev9 at the same step shows instability (Fig. 2.15 left vs middle).

## Common Strategies

- Right tool when stiffness is "real-axis only" — large parabolic and reaction-diffusion problems where the spectrum lies near the negative real axis.
- Compared against [[entities/rkc]] and ROCK4 (Abdulle) as the Chebyshev-method family in Sect. IV.10.

## Related Entities

- V.I. Lebedev — originator.
- A. Medovikov — author of DUMKA3.
- [[entities/rkc]] — sibling Chebyshev code by Sommeijer.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
