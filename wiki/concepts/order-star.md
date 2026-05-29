---
title: Order Star
type: claim
id: claim-order-star
tags:
- ode
- numerical-integration
- stability
- mathematical-tool
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

The order star of a rational approximation R(z) to e^z is the open subset of the complex plane on which |R(z)/e^z| > 1 (Wanner, Hairer, Nørsett 1978). This *relative* level set, comparing R to e^z rather than to 1, has a topology — number of fingers crossing the origin, number of poles, number of zeros — that encodes order, stability, and barriers in a single picture.

## How It Works

The key topological theorem: if R(z) approximates e^z to order p at the origin, then exactly p + 1 fingers of the order star (regions where |R/e^z| > 1) meet at z = 0, with alternating "in" / "out" character. A-stability of R is equivalent to no order-star finger crossing the imaginary axis; L-stability requires R(∞) = 0, equivalent to no finger extending to infinity. Counting fingers proves: the [[concepts/dahlquist-barrier]] (A-stable LMS order ≤ 2); the [[concepts/daniel-moore-conjecture]] (A-stable s-stage IRK order ≤ 2s); real-pole order bounds (Nørsett–Wolfbrandt); and Ehle's conjecture (the (k, j) Padé approximation is A-stable iff k ≤ j ≤ k + 2). The Chapter V extension to Riemann surfaces (V.4) lifts the same finger-counting machinery to multi-valued characteristic functions of multistep methods.

## Key Parameters

- Order p of the approximation.
- Number of "in" fingers at the origin (= p + 1).
- Pole / zero distribution of R.
- Behaviour at infinity.

## When To Use

- Proving impossibility theorems (order barriers) for numerical methods.
- Verifying A-, L-, or A(α)-stability of rational stability functions.
- Classifying Padé approximants and constructing optimal-order stable methods.
- Understanding the Riemann-surface structure of multistep characteristic equations.

## Risks & Pitfalls

- The argument is qualitative (counting fingers); quantitative error constants require additional analysis.
- For multistep methods the lift to Riemann surfaces is technical and the "finger" interpretation generalises non-trivially.
- Order stars say nothing about nonlinear ([[concepts/b-stability]]) behaviour.

## Related Concepts

- [[concepts/stability-function]]
- [[concepts/a-stability]]
- [[concepts/l-stability]]
- [[concepts/pade-approximation]]
- [[concepts/dahlquist-barrier]]
- [[concepts/daniel-moore-conjecture]]
- [[concepts/riemann-surface]]
- [[concepts/property-c]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
