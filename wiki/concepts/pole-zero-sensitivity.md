---
title: Pole and Zero Sensitivity
type: claim
id: claim-pole-zero-sensitivity
tags:
- sensitivity
- analog
- ac
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt
confidence:
  base: 0.85
---

## Definition

The sensitivity of a polynomial root z (zero of the numerator or pole of the denominator) with respect to a parameter h is dz/dh = -(dP/dh)/(dP/ds)|_{s=z}, valid for simple roots. The normalized form is S_h^z = (h/z)(dz/dh).

## How It Works

For a complex root z = a + jb, the sensitivities of the real and imaginary parts are:
- S_h^a = Re[h dz/dh] (semi-normalized form, valid even when a = 0).
- S_h^b = Im[h dz/dh].

These give insight into how the root migrates in the complex plane as h changes — useful for stability analysis (movement toward or away from the imaginary axis).

## Key Parameters

- Multiplicity of the root (formula above assumes simple).
- Position of the root in the complex plane.

## When To Use

- Stability analysis: tracking left-half-plane vs. right-half-plane pole migration.
- Filter design where pole/zero locations directly determine response shape.
- Foundation for Q and omega_0 sensitivities.

## Risks & Pitfalls

- Formula breaks down at multiple (repeated) roots; perturbation theory for multiple roots requires fractional-order expansions.
- Numerical computation of dP/dh and dP/ds at the root must be performed carefully.

## Related Concepts

- [[concepts/poles-and-zeros]]
- [[concepts/sensitivity-analysis]]
- [[concepts/q-omega-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
