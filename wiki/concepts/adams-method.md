---
title: Adams Method
type: claim
id: claim-adams-method
tags:
- ode
- numerical-integration
- multistep
- nonstiff
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

An Adams method is a [[concepts/linear-multistep-methods|linear multistep]] integrator based on integrating an interpolating polynomial of f(x, y) past values: y_{n+k} − y_{n+k−1} = h ∑_{j=0}^k β_j f_{n+j}. The *explicit* Adams–Bashforth method (Bashforth–Adams 1883) uses β_k = 0; the *implicit* Adams–Moulton method uses β_k ≠ 0 and is one order higher for the same number of steps.

## How It Works

The first characteristic polynomial is ρ(ζ) = ζ^k − ζ^{k−1} (so ρ(1) = 0 with simple zero — zero-stable for every k). The β_j weights come from ∫_{k−1}^k ℓ_j(τ) dτ where ℓ_j is the Lagrange interpolant at the k + 1 nodes. Adams methods have *tiny* [[concepts/stability-region]]s in the left half plane — completely unsuitable for stiff problems. Used as a [[concepts/predictor-corrector-method]] (PECE: predict with Adams–Bashforth, evaluate, correct with Adams–Moulton, evaluate), the stability region shrinks further (Chase 1962, Crane–Klopfenstein 1965, Krogh 1966). Code-wise, Adams families dominate the nonstiff regime: LSODE, DDASSL, and Hindmarsh's ODEPACK use Adams of order 1–12 for nonstiff problems.

## Key Parameters

- Step count k (orders k for Adams–Bashforth, k+1 for Adams–Moulton).
- Coefficients β_j (tabulated, signed pattern).
- Stability-interval length on the real and imaginary axes.

## When To Use

- Nonstiff ODE integration where high order is helpful.
- Smooth, oscillatory, or weakly damped problems.
- Code lineages that variable-order across the integration (LSODE, DDASSL nonstiff branch).

## Risks & Pitfalls

- Useless on stiff problems: the [[concepts/dahlquist-barrier]] caps A-stable order at 2, and Adams stability regions are far smaller than BDF's.
- Variable step / variable order implementation is intricate (interpolation polynomial maintenance).
- PECE iteration shrinks the stability region — multiple correctors (PEC, PECEC) trade work for stability margin.

## Related Concepts

- [[concepts/linear-multistep-methods]]
- [[concepts/predictor-corrector-method]]
- [[concepts/gear-bdf]]
- [[concepts/dahlquist-barrier]]
- [[concepts/stability-region]]
- [[concepts/nystrom-method]]
- [[concepts/blended-multistep-method]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
