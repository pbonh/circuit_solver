---
title: "Kharitonov Bounds (Robust Stability)"
type: concept
tags: [control-theory, robust, foundational, analog]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations.txt"]
confidence: low
---

## Definition

Kharitonov's theorem states that a family of polynomials with coefficients in independent intervals is stable (all roots in the left half-plane) iff four specially constructed extremal polynomials are stable. Kharitonov-style bounds extend this to frequency-domain magnitude and phase envelopes for interval-coefficient transfer functions.

## How It Works

For an interval polynomial `p(s) = [a0_lo, a0_hi] + [a1_lo, a1_hi] s + ...`, four extremal Kharitonov polynomials (alternating lo/hi pattern) suffice to certify Hurwitz stability of the whole family. Magnitude/phase frequency envelopes are derived by evaluating the family at `s = j omega` and bounding numerator/denominator independently.

## Key Parameters

- Coefficient interval bounds.
- Coupling structure (independent vs. correlated coefficients).

## When To Use

- Quick robust stability check under coefficient uncertainty.
- Worst-case frequency response when correlations are weak or unknown.

## Risks & Pitfalls

- Assumes independent coefficient intervals; over-conservative for correlated parameters.
- Does not directly handle transfer-function pole-zero coupling.

## Related Concepts

- [[concepts/performance-bound-analysis]]
- [[concepts/interval-arithmetic]]
- [[concepts/process-variation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations]]
