---
title: "Discrete and Fast Fourier Transform (DFT/FFT)"
type: concept
tags: [foundational, math, well-established, numerical]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt"]
confidence: high
---

## Definition

The discrete Fourier transform (DFT) is a linear transformation between two sequences of n+1 values, defined by a_j = (1/(n+1)) sum_i y_i w^{-ij} with w = exp(2 pi j / (n+1)) and its inverse y_i = sum_j a_j w^{ij}. The fast Fourier transform (FFT) computes the DFT in O((n+1) log(n+1)) operations when n+1 is a power of 2; the Winograd FFT generalizes to arbitrary sizes.

## How It Works

The DFT arises naturally as the solution of polynomial interpolation on the unit circle: if y_i = P(x_i) at x_i = w^i, then the coefficients a_j of P satisfy a Vandermonde system that decouples into DFT form. The orthogonality X^* X = (n+1) I gives the inverse explicitly.

In Vlach & Singhal Section 7.5, the DFT is used to convert sampled values of N(s_i) and D(s_i) into polynomial coefficients of the network function. A combined trick: form c_i = N_i + j D_i and take a single DFT — real parts of the result are numerator coefficients, imaginary parts are denominator coefficients.

## Key Parameters

- n+1 (number of samples = polynomial degree + 1).
- Whether n+1 is highly composite (better for FFT speed).
- Single-precision vs. double-precision arithmetic.

## When To Use

- Polynomial interpolation on the unit circle.
- Spectral analysis of sampled signals.
- Symbolic function generation in CAD.

## Risks & Pitfalls

- For very large n, finite-precision arithmetic accumulates error proportional to log(n).
- Aliasing if the sample rate is too low relative to signal bandwidth.

## Related Concepts

- [[concepts/symbolic-function-generation]]
- [[concepts/interpolation-condition-number]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
