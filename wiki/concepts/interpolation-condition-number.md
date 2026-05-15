---
title: "Interpolation Condition Number"
type: concept
tags: [foundational, math, numerical, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt"]
confidence: medium
---

## Definition

The condition number K(X) of an interpolation point set is K(X) = sqrt(omega_max / omega_min), where omega_max and omega_min are the largest and smallest eigenvalues of X^* X. It measures how much errors in the data y are amplified into errors in the polynomial coefficients a. K(X) = 1 is optimal; for points uniformly distributed on the unit circle, K(X) = 1 exactly.

## How It Works

From the interpolation equation X a = y, perturbations in X or y propagate to a with amplification bounded by K(X). For real interpolation, K(X) grows rapidly with the number of points (e.g., 10^7 or more for 20 points on [-1, 1]). Vlach & Singhal Fig. 7.4.1 shows: Chebyshev points (best real choice) still much worse than the unit circle.

## Key Parameters

- Spacing/distribution of interpolation points.
- Polynomial degree.
- Numerical precision used.

## When To Use

- Choosing where to sample for symbolic function generation.
- Designing custom interpolation schemes for CAD.

## Risks & Pitfalls

- Naive equidistant real interpolation is catastrophically ill-conditioned (Runge phenomenon).
- Even Chebyshev points (best on a finite real interval) are worse than the unit circle.

## Related Concepts

- [[concepts/symbolic-function-generation]]
- [[concepts/dft-fft]]
- [[concepts/condition-number]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
