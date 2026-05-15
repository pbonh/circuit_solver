---
title: "Peano Kernel"
type: concept
tags: [ode, numerical-integration, error-analysis, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

The Peano kernel of a linear functional L (typically a truncation-error functional of a numerical method) is the kernel K_p(t) appearing in the Peano representation L[f] = ∫ K_p(t) f^{(p+1)}(t) dt, valid when L annihilates polynomials of degree ≤ p. The kernel exposes precisely which derivatives of f contribute to the error, with what sign and magnitude.

## How It Works

For a [[concepts/linear-multistep-methods]] method, the truncation error functional L[y] = ∑ α_i y(x_{n+i}) − h ∑ β_i y'(x_{n+i}) is a linear functional on smooth y. If the method has order p, L kills polynomials up to degree p, and the Peano theorem gives L[y] = h^{p+1} ∫ K_p(t) y^{(p+1)}(x_n + h t) dt with K_p the *Peano kernel*. The L^∞ norm of K_p is a sharper version of the [[concepts/error-constant]]: it bounds the truncation error in terms of ‖y^{(p+1)}‖_∞. The Jeltsch–Nevanlinna accuracy barrier (Theorem 2.6, Hairer–Wanner V) uses the L^∞ Fourier–Peano kernel to bound the smallest achievable error constant of any method whose [[concepts/stability-region]] contains a tangent disc of radius r.

## Key Parameters

- Method order p.
- Kernel K_p(t).
- ‖K_p‖_∞ (or L^1) bounds.

## When To Use

- Sharp truncation-error analysis of LMS / RK methods.
- Proving accuracy barrier theorems linking stability and error constants.
- Comparing error constants across methods of the same order.

## Risks & Pitfalls

- The kernel can be sign-changing; ‖K_p‖_1 vs. ‖K_p‖_∞ give different bounds.
- For non-smooth right-hand sides the Peano representation is invalid past the regularity of f.

## Related Concepts

- [[concepts/error-constant]]
- [[concepts/linear-multistep-methods]]
- [[concepts/dahlquist-barrier]]
- [[concepts/property-c]]
- [[concepts/order-star]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
