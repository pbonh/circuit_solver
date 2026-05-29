---
title: RKC
type: entity
id: entity-rkc
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

RKC ("Runge-Kutta-Chebyshev") is B.P. Sommeijer's research code (1991) for the van der Houwen-Sommeijer family of stabilised explicit Runge-Kutta methods (Sect. IV.2). It is available by netlib mail: `send rkc.f from ode`.

## Characteristics

- Builds on scaled-and-shifted [[concepts/chebyshev-method]] polynomials and the three-term recursion (eq. 2.49') for the internal stages (Bakker 1973 / van der Houwen-Sommeijer 1980).
- Tunable damping parameter (Verwer-Hundsdorfer-Sommeijer 1990); Fig. 2.16 shows the stability function and domain for s = 9 stages, c = 0.15.
- Explicit method — no Jacobian/linear-solver overhead — but stability is limited to spectra clustered along the negative real axis.

## Common Strategies

- The go-to explicit code in Sect. IV.10 experiments where the problem is "mildly stiff" / parabolic; e.g., RKC gives excellent results at low precision on the KS / Kuramoto-Sivashinsky-type problems where DOPRI5 takes more than 30 seconds.

## Related Entities

- B.P. Sommeijer — author.
- [[entities/dumka]] — alternative Chebyshev-method code by Lebedev.
- van der Houwen — co-developer of the underlying method.

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
