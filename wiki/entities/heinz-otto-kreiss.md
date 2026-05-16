---
title: "Heinz-Otto Kreiss"
type: entity
tags: [ode, numerical-integration, foundational, stability]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt", "raw/solving_ordinary_differential_equations_ii/_txt/bibliography.txt"]
confidence: medium
---

## Overview

Heinz-Otto Kreiss is the Swedish-American numerical analyst whose 1962 paper "Über die Stabilitätsdefinition für Differenzengleichungen die partielle..." proved the [[concepts/kreiss-matrix-theorem]]. The theorem — an equivalence between power-boundedness of a matrix family and a resolvent condition — is the central tool of Sect. V.7 for proving uniform power boundedness of the companion matrices that arise from multistep methods.

## Characteristics

- Kreiss (1962) — foundational paper whose dedication notes that his Stockholm dissertation faculty opponent G. Dahlquist first raised the question of a stability definition for difference equations.
- The Kreiss matrix theorem is one of three workhorses Hairer-Wanner cite for nonlinear convergence proofs in Vol. II: G-stability, the Kreiss matrix theorem, and the multiplier technique (Sect. V opening summary).
- The Kreiss problem (subject index 542) and the resolvent condition (subject index 332) are used throughout Sect. V.7-V.8 to bound global error in terms of local error for general linear methods.
- LeVeque & Trefethen (1984) — "On the resolvent condition in the Kreiss matrix theorem" — refines the constants for finite-dimensional cases (cited as the LeVeque-Trefethen conjecture, subject index 730).

## Common Strategies

- Reduction of multistep stability questions to resolvent estimates on the companion matrix.
- Generalisation of A-stability to matrix-valued and operator-valued settings.

## Related Entities

- [[entities/germund-dahlquist]] — Stockholm faculty-opponent connection.
- R.J. LeVeque, L.N. Trefethen — refined the matrix-theorem constants.
- M.N. Spijker — proved a related conjecture (bibliography 1108).

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
