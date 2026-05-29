---
title: Semi-Normalized Sensitivity
type: claim
id: concepts/semi-normalized-sensitivity
tags:
- sensitivity
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Two semi-normalized sensitivities accommodate edge cases of the normalized form:
- S_h^bar = h (dF/dh) — useful when F = 0 (e.g., sensitivity of a zero of a polynomial).
- S_h^tilde = (1/F)(dF/dh) — useful when h = 0 (e.g., sensitivity to a parasitic).
When both F = 0 and h = 0, the differential D_h^F = dF/dh must be used.

## How It Works

These definitions appear in Vlach & Singhal Section 5.1, Eqs. 5.1.3 and 5.1.4. They preserve dimensional information from the unnormalized factor while still providing a useful scale for comparison.

## Key Parameters

- Magnitude of F or h (which is zero).
- Limit behavior of the parameter at the nominal point.

## When To Use

- Sensitivity of pole or zero positions when they sit at the origin.
- Sensitivity to parasitic elements whose nominal value is zero.
- Sensitivity to OPAMP gain via B = -1/A (B = 0 for ideal OPAMP).

## Risks & Pitfalls

- Less standardized than the normalized form; the form to use depends on the context.
- Mixing forms in the same comparison can mislead.

## Related Concepts

- [[concepts/normalized-sensitivity]]
- [[concepts/parasitic-sensitivity]]
- [[concepts/gain-sensitivity-product]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
