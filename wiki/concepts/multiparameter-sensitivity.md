---
title: Multiparameter Sensitivity (Worst-Case, Tracking, Statistical)
type: claim
id: concepts/multiparameter-sensitivity
tags:
- sensitivity
- analog
- well-established
- statistics
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Multiparameter sensitivity measures aggregate the effects of simultaneous variations in many parameters on a single response quantity F. Vlach & Singhal define three:
- Worst-Case Multiparameter Sensitivity: WCMS = sum_i |S_{h_i}^F|.
- Multiparameter Tracking Sensitivity (per element type k): MTS_k = |sum_{i in k} S_{h_i}^F|.
- Multiparameter Statistical Sensitivity: MSS = [sum_i (S_{h_i}^F)^2]^{1/2}.

## How It Works

If every parameter has tolerance t:
- Worst case: |delta F / F| ≤ t * WCMS — assumes all delta h_i are at the extreme of their range with signs aligned with sensitivity signs.
- Tracking: |delta F / F| ≤ sum_k t_k * MTS_k — assumes elements of the same type vary identically (e.g., IC capacitors track).
- Statistical: assuming statistically independent normally distributed delta h_i with variance sigma^2 each, the variance of delta F / F is sigma^2 * (MSS)^2, giving 1-sigma, 2-sigma, 3-sigma confidence intervals.

## Key Parameters

- Tolerance t per parameter.
- Distribution (uniform: variance t^2/3; normal: variance t^2/9 when 3-sigma = t).
- Correlations among parameters (perfect tracking, independence, etc.).

## When To Use

- Tolerance analysis for production design.
- Comparing yield among competing designs (lower MSS → higher yield).
- Allocating tighter or looser tolerances among components.

## Risks & Pitfalls

- WCMS is usually overly pessimistic; MSS is more realistic for high-volume production.
- Tracking assumptions must reflect the actual process (IC vs. discrete).
- Linear-sensitivity analysis breaks down for large perturbations (Q in Example 5.2.1 changed by 38% predicted vs. 62% actual at 1% individual tolerance).

## Related Concepts

- [[concepts/sensitivity-analysis]]
- [[concepts/normalized-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
