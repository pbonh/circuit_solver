---
title: "Stiff Stability"
type: concept
tags: [transient, numerical-integration, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt"]
confidence: medium
---

## Definition

Stiff stability (Gear, 1971) is a relaxation of A-stability adequate for stiff problems. A method is stiffly stable if its region of absolute stability includes a strip {q in q-plane : Re q < -a, |Im q| < d} for some constants a, d > 0, plus a portion of the left half-plane. The Dahlquist barrier says no A-stable LMS has order > 2, but stiffly-stable LMS methods (BDF) exist up to order 6.

## How It Works

For BDF orders 1-2: A-stable (full left half-plane stable). For BDF orders 3-6: stiffly stable (some sliver near the imaginary axis in the left half-plane is outside the stability region, but the rest of the left half-plane is). For orders 7 and beyond, even stiff stability fails — these are not used.

In practice, stiff stability is sufficient for circuit simulation because the unstable region is near the imaginary axis, where stiff systems do not have eigenvalues.

## Key Parameters

- a (stability strip half-width to the left of the imaginary axis).
- d (height of the strip).
- Method order.

## When To Use

- Stiff system integration where A-stability is too restrictive.
- BDF methods of orders 3-6.

## Risks & Pitfalls

- Lightly damped oscillating systems may have eigenvalues near the imaginary axis, in the stability-region gap. Such systems need A-stable methods (trapezoidal).

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/gear-bdf]]
- [[concepts/stiff-systems]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
