---
title: Root Locus Curve
type: claim
id: concepts/root-locus-curve
tags:
- ode
- numerical-integration
- multistep
- stability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The root locus curve of a [[concepts/linear-multistep-methods|linear multistep]] method with characteristic polynomials (ρ, σ) is the image of the unit circle under the map μ = ρ(ζ) / σ(ζ) for |ζ| = 1. It is the boundary of the [[concepts/stability-region]] in the μ = h λ plane.

## How It Works

By definition of zero-stability, all roots ζ of ρ(ζ) − μ σ(ζ) = 0 must lie inside the closed unit disk for μ in the stability region. As μ varies, the roots move continuously and cross the unit circle exactly at points μ on the locus curve. The curve is plotted by sweeping ζ = e^{iθ} for θ ∈ [0, 2π] and evaluating ρ(e^{iθ})/σ(e^{iθ}). For [[concepts/gear-bdf]] methods of orders k = 1..6, the curves cover progressively smaller regions of the imaginary axis, with the curve for k = 7 self-intersecting — the geometric reason BDF7 is unusable.

## Key Parameters

- Characteristic polynomials (ρ, σ).
- Step count k.
- Order p.

## When To Use

- Visualising the stability region of a multistep method.
- Diagnosing zero-instability or weak instability (curve passes through the origin or crosses itself).
- Selecting an [[concepts/a-alpha-stability]] angle by inspection.

## Risks & Pitfalls

- The curve is the *boundary* of the stability region — the interior is the side where |R(z)| ≤ 1 for one-step methods, or where all auxiliary roots stay inside the unit disk for multistep.
- For multistep methods with parasitic roots near |ζ| = 1, the curve can self-intersect, signalling weak instability.
- Visual inspection can mislead at branch points; cross-check with explicit root computation.

## Related Concepts

- [[concepts/stability-region]]
- [[concepts/stability-function]]
- [[concepts/linear-multistep-methods]]
- [[concepts/gear-bdf]]
- [[concepts/order-star]]
- [[concepts/riemann-surface]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
