---
title: "Objective Function"
type: concept
tags: [optimization, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt"]
confidence: high
---

## Definition

The objective function F(x) is a scalar function of the design variables x = (x_1, ..., x_n) that the optimization procedure minimizes (or maximizes). The choice of F encodes the design goal. In CAD, common choices include:
- Sum of squared deviations from a desired response: F = sum_i (R(x, omega_i) - R_spec(omega_i))^2.
- Worst-case deviation: F = max_i |R - R_spec|.
- Negative gain or attenuation at a specified frequency.

## How It Works

The objective function depends on the design variables x indirectly through the network response. For each evaluation:
1. Set network elements to x.
2. Solve the network equations (DC, AC, transient).
3. Extract response of interest.
4. Compute F(x) from the response.

Gradient of F with respect to x is computed via the adjoint sensitivity method of Chapter 6, supplied to the optimization algorithm.

## Key Parameters

- Choice of error metric (L_2, L_infinity, weighted variants).
- Frequency or time points evaluated.
- Weighting of multiple objectives.
- Regularization terms (e.g., penalties on component count).

## When To Use

- Defining "what you want" precisely enough for an algorithm.
- Multi-criteria design (combine multiple objectives via weights or constraints).

## Risks & Pitfalls

- A poorly-chosen objective can produce mathematically correct but physically undesirable solutions.
- Local-minimum traps depend strongly on objective formulation.
- Continuity / smoothness of F affects algorithm performance.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
