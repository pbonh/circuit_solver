---
title: Q and omega_0 Sensitivity
type: claim
id: claim-q-omega-sensitivity
tags:
- sensitivity
- analog
- ac
- well-established
- filter
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt
confidence:
  base: 0.85
---

## Definition

For a complex pole pair p, p_bar with coordinates (a, b), the quality factor Q and natural frequency omega_0 satisfy:
- Q = -omega_0 / (2a).
- omega_0^2 = a^2 + b^2.

Their normalized sensitivities relate to the position sensitivities S_h^a and S_h^b:
- S_h^Q = S_h^omega_0 - S_h^a.
- S_h^omega_0 = (1/omega_0^2)(a^2 S_h^a + b^2 S_h^b).

In high-Q circuits a^2 << b^2 and S_h^omega_0 ≈ S_h^b.

## How It Works

Q and omega_0 are derived parameters of a second-order section; designers find them more intuitive than complex pole coordinates. The sensitivities are computed from the pole-position sensitivities of Chapter 5 Section B and provide a compact characterization of how tight the manufacturing tolerances on each element must be to achieve a target Q.

## Key Parameters

- a (real part of pole) — controls damping.
- b (imaginary part) — controls oscillation frequency.
- Q — quality factor of the section.
- omega_0 — natural frequency.

## When To Use

- Sensitivity analysis of biquad-based active filters.
- Tolerance budgeting in filter design.
- Comparison of different active-network realizations of the same biquad transfer function.

## Risks & Pitfalls

- High-Q approximations introduce error when a is not actually << b.
- Q and omega_0 are defined per pole pair; networks with many poles need pair-by-pair analysis.

## Related Concepts

- [[concepts/pole-zero-sensitivity]]
- [[concepts/poles-and-zeros]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
