---
title: Symbolic Function Generation (Interpolation-Based)
type: claim
id: claim-symbolic-function-generation
tags:
- analog
- ac
- well-established
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt
confidence:
  base: 0.85
---

## Definition

Symbolic function generation computes the rational network function F(s) = N(s)/D(s) by polynomial interpolation. Given numerical values for all element values, the function is recovered as a polynomial ratio by solving the system equations at several frequencies and using the DFT to extract polynomial coefficients.

## How It Works

Algorithm (Section 7.5 of Vlach & Singhal):
1. Set up TX = W; ensure no 1/s terms.
2. Estimate degree n_0 (at most capacitors + inductors).
3. Choose n_0 + 1 points uniformly spaced on the unit circle: s_k = exp(2 pi j k / (n_0 + 1)).
4. For each s_k, LU-factor T(s_k), then:
   - D(s_k) = product of L diagonal entries.
   - F(s_k) by forward/back substitution.
   - N(s_k) = D(s_k) F(s_k).
5. Apply DFT to (s_k, N_k) and (s_k, D_k) to recover polynomial coefficients.

For sparse circuits, this is faster than the QZ algorithm.

## Key Parameters

- Estimated polynomial degree n_0.
- Number of unit-circle points (must be > n_0).
- Frequency scaling of the network (max|b_i|/min|b_i| controls numerical accuracy).

## When To Use

- Generating the rational form of a transfer function for symbolic-style analysis.
- Time-domain response via Laplace inversion (Chapter 10).
- Faster than QZ for large sparse networks.

## Risks & Pitfalls

- Coefficient cancellation between superfluous high-order terms when the true degree is overestimated; check by looking for near-zero leading coefficients.
- Poor scaling produces noisy coefficients and false right-half-plane poles in the 9th-order Cauer example.
- A pole coincident with a unit-circle point produces a singular T; increasing the number of points or re-scaling the network resolves this.

## Related Concepts

- [[concepts/dft-fft]]
- [[concepts/interpolation-condition-number]]
- [[concepts/qz-algorithm]]
- [[concepts/network-function]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
