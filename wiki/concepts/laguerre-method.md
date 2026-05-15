---
title: "Laguerre's Method (Polynomial Root Finder)"
type: concept
tags: [math, numerical, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt"]
confidence: medium
---

## Definition

Laguerre's method is an iterative algorithm for finding roots of polynomials. It is more globally convergent than Newton-Raphson and can converge to complex roots from real initial estimates. The update formula uses both the first and second derivatives of the polynomial.

## How It Works

x^{k+1} = x^k - n P_n(x^k) / (P_n'(x^k) +/- sqrt(H^k))

where H^k = (n - 1)[(n - 1)(P_n')^2 - n P_n P_n''] (Eq. 7.6.3). The sign of the square root is chosen to minimize |x^{k+1} - x^k|.

H^k may become negative, producing an imaginary correction; this allows convergence to complex roots from real initial estimates. Recommended initial estimate: x^0 = 0, so small-magnitude roots are extracted first (preserving accuracy in subsequent deflations).

## Key Parameters

- Polynomial degree n.
- Convergence tolerance epsilon.
- Maximum iteration count.

## When To Use

- Polynomial root-finding when Newton-Raphson is unreliable.
- Educational introduction to root-finding beyond Newton.

## Risks & Pitfalls

- More expensive per iteration than Newton-Raphson (requires P_n'').
- Newer methods (Jenkins-Traub) are more accurate and robust.
- Deflation accumulates error; roots should be refined on the original polynomial after extraction.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/root-refinement]]
- [[concepts/poles-and-zeros]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
