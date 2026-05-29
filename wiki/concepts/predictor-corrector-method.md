---
title: Predictor–Corrector Method
type: claim
id: claim-predictor-corrector-method
tags:
- ode
- numerical-integration
- multistep
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

A predictor–corrector method combines an explicit (predictor) and an implicit (corrector) multistep formula into a single composite step. The predictor produces a first guess ŷ_{n+k}; the corrector refines it. Notation: PECE applies the corrector once (P → E → C → E); PEC stops after the first correction; PECEC applies it twice.

## How It Works

The classical pairing is Adams–Bashforth (predictor) plus Adams–Moulton (corrector) of the same order. Convergence of the corrector iteration requires h L_f < some method-specific bound; in practice one correction is sufficient when h is small. The composite method's [[concepts/stability-region]] is strictly smaller than the corrector's alone — successive corrections grow the region but never reach the implicit corrector's region (Chase 1962, Crane–Klopfenstein 1965, Krogh 1966). For stiff problems the bound on h L_f is violated and the corrector iteration diverges — a fundamental reason PECE schemes fail on stiffness.

## Key Parameters

- Predictor formula and order p_P.
- Corrector formula and order p_C (typically p_C = p_P or p_C = p_P + 1).
- Number of correction iterations (1 for PECE, 2 for PECEC).
- Convergence test on the corrector residual.

## When To Use

- Nonstiff ODE codes seeking implicit-method accuracy with explicit-method cost.
- Adams-family lineage (Hindmarsh LSODE nonstiff branch).
- Educational / classical numerical-analysis settings.

## Risks & Pitfalls

- Divergence of the corrector iteration on stiff problems.
- Composite stability region smaller than the corrector's alone — never as good as a fully implicit method.
- Order of the composite is min(p_P + #corrections, p_C); choose accordingly.

## Related Concepts

- [[concepts/adams-method]]
- [[concepts/linear-multistep-methods]]
- [[concepts/stability-region]]
- [[concepts/nystrom-method]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
