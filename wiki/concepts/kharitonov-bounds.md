---
title: Kharitonov Bounds (Robust Stability)
type: claim
id: claim-kharitonov-bounds
tags:
- control-theory
- robust
- foundational
- analog
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations.txt
confidence:
  base: 0.65
---

> Advanced Symbolic Analysis for VLSI Systems Chapter 11 ("Performance Bound Analysis of Analog Circuits Considering Process Variations") cites Kharitonov in the context of worst-case frequency-domain analysis: "Recently, worst-case analysis of linearized analog circuits in frequency domain has been proposed [158], where Kharitonov's functions [93] were applied to obtain the performance bounds in frequency domain, but no systematic method was proposed to obtain variational transfer functions. This was later improved by [78], where symbolic analysis approach was applied to derive exact transfer functions and affine interval method was used to compute variational transfer functions. However, the affine interval method can lead to over-conservative results."

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
