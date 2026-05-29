---
title: Normalized Sensitivity
type: claim
id: claim-normalized-sensitivity
tags:
- sensitivity
- foundational
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt
confidence:
  base: 0.85
---

## Definition

The normalized sensitivity of a network function F with respect to a parameter h is S_h^F = (h/F) (dF/dh) = (d ln F)/(d ln h). It measures the relative change in F per relative change in h and is dimensionless, enabling fair comparison across different parameters and networks.

## How It Works

For T = N/D, S_h^T = S_h^N - S_h^D. For T = |T| e^{j phi}, S_h^|T| = Re S_h^T and S_h^phi = (1/phi) Im S_h^T (the latter is sometimes left unnormalized as Im S_h^T). The normalized form is exact for differential changes and a good approximation for small finite changes via delta F/F ~ S_h^F (delta h / h).

## Key Parameters

- Frequency at which sensitivity is evaluated (sensitivity is generally s-dependent).
- Type of F (magnitude, phase, pole position, Q, omega_0).
- Type of h (element value, gain, frequency, temperature).

## When To Use

- Comparing alternative network designs with the same nominal response.
- Tolerance budgeting: |delta F / F| ≤ |S_h^F| (delta h / h) for small variations.
- As gradient input to optimization algorithms.

## Risks & Pitfalls

- Undefined when F = 0 or h = 0 (use semi-normalized variants).
- Only valid for small variations; large changes need large-change sensitivity (Chapter 8).
- Sensitivity at a single frequency may not predict response behavior at other frequencies.

## Related Concepts

- [[concepts/sensitivity-analysis]]
- [[concepts/semi-normalized-sensitivity]]
- [[concepts/multiparameter-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
