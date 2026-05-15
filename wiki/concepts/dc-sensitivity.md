---
title: "DC Sensitivity"
type: concept
tags: [dc, sensitivity, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt"]
confidence: medium
---

## Definition

DC sensitivity computes how the DC operating point v_n changes with respect to network parameters p: dv_n/dp. After solving the nonlinear DC equations f(v_n, p) = 0, the sensitivity is computed by implicit differentiation: M dv_n/dp + df/dp = 0, where M = df/dv_n is the converged Jacobian.

## How It Works

The Jacobian M is already factored from the final Newton-Raphson iteration. So dv_n/dp = -M^{-1} (df/dp) costs only one forward/back substitution per parameter (or one per output via the adjoint method of Chapter 6).

This is essentially free given that the DC solve has already been done. Tolerance budgeting for analog ICs uses these sensitivities to identify which components must be matched tightly.

## Key Parameters

- Number of parameters (for adjoint method, irrelevant).
- Number of outputs of interest.
- Use of adjoint vs. direct sensitivity formulation.

## When To Use

- Tolerance analysis of analog ICs.
- Gradient computation for DC-objective optimization.
- Yield-aware design.

## Risks & Pitfalls

- Requires converged DC solution; cannot be computed without it.
- The Jacobian at convergence may be ill-conditioned for devices in steep regions.

## Related Concepts

- [[concepts/dc-analysis]]
- [[concepts/sensitivity-analysis]]
- [[concepts/transpose-system-method]]
- [[concepts/jacobian-matrix]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
