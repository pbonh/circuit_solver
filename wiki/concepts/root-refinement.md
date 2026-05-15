---
title: "Root Refinement (System-Matrix-Based)"
type: concept
tags: [foundational, ac, well-established, numerical]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt"]
confidence: medium
---

## Definition

Root refinement uses Newton-Raphson iteration on the original system equations (rather than on a polynomial approximation) to correct estimates of poles and zeros obtained by polynomial-coefficient methods. It is more accurate than polynomial root-finding because it bypasses the coefficient-error propagation analyzed in Eq. 7.7.2.

## How It Works

For a zero z: solve T(z^k) X = W and T^T(z^k) X^a = -d, then update z^{k+1} = z^k - F(z^k)/((X^a)^T C X). Convergence is local-quadratic.

For a pole p: factor T(p^k) = LU; the (n,n) entry l_{nn} approaches zero as p^k approaches p. Solve L^T y = l_{nn} e_n (using y_n = 1) and U z = e_n; update p^{k+1} = p^k - l_{nn}/(y^T C z) by Newton-Raphson on l_{nn}(s) = 0.

At convergence, the auxiliary vectors (X, X^a) or (y, z) are exactly those needed for root sensitivity, so root sensitivities are obtained "for free."

## Key Parameters

- Initial estimate (from polynomial root-finding).
- Newton-Raphson convergence tolerance.
- Whether pivoting is needed at the pole.

## When To Use

- Post-processing after polynomial-based pole/zero estimation.
- High-precision pole/zero determination.
- When poles/zeros must be paired with sensitivity computations.

## Risks & Pitfalls

- Requires a good initial estimate; can diverge if too far from the root.
- Multiple roots break the simple Newton formulation.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/pole-sensitivity-singular-matrix]]
- [[concepts/symbolic-function-generation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
