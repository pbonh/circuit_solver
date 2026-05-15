---
title: "Interval (and Affine) Arithmetic"
type: concept
tags: [foundational, numerical, statistical, bounds]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations.txt"]
confidence: medium
---

## Definition

Interval arithmetic replaces every scalar with a range `[a_lo, a_hi]` and defines arithmetic operations that produce a range provably containing all results. Affine arithmetic tracks first-order dependencies on shared "noise symbols" to reduce over-conservatism.

## How It Works

Interval addition: `[a,b] + [c,d] = [a+c, b+d]`. Interval multiplication: take min/max of all corner products. Affine arithmetic: each quantity is an affine function `x0 + sum x_i epsilon_i`; operations propagate the coefficients, and any new uncertainty introduces a fresh noise symbol.

## Key Parameters

- Number of noise symbols (affine arithmetic).
- Operation order (interval results depend on the expression form).

## When To Use

- Conservative worst-case bound estimation.
- Process-variation propagation when distribution shapes are unknown.

## Risks & Pitfalls

- Naive interval arithmetic explodes the result range (dependency / wrapping problem).
- Affine arithmetic mitigates this but is still over-conservative; this chapter notes that DDD + nonlinear optimization gives tighter bounds.

## Related Concepts

- [[concepts/performance-bound-analysis]]
- [[concepts/kharitonov-bounds]]
- [[concepts/process-variation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations]]
