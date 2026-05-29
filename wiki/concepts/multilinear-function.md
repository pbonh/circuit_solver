---
title: Multilinear (Bilinear) Network Function
type: claim
id: claim-multilinear-function
tags:
- analog
- well-established
- foundational
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt
confidence:
  base: 0.65
---

## Definition

At a fixed complex frequency s, a network function F(delta_1, ..., delta_m) (with delta_i = element perturbations) is multilinear: F is bilinear in each delta_i separately. Both numerator and denominator are multilinear forms with at most 2^m coefficients in their multivariate expansion.

## How It Works

The bilinearity follows from the rank-one nature of each element's contribution to the system matrix. Symbolic analysis exploits this structure: all 2^m coefficients of N(delta_1, ..., delta_m) and D(delta_1, ..., delta_m) are determined by the (m+1) x (m+1) matrix F_hat from large-change sensitivity. Specifically:
- dD/d(delta_{i1} ... delta_{il}) at delta=0 = det T_0 * det F_subset(i1, ..., il).
- dN/d(delta_{i1} ... delta_{il}) at delta=0 = det T_0 * det F_subset(i1, ..., il, m+1).

For m = 2, F has the form: F(delta_1, delta_2) = (a + a_1 delta_1 + a_2 delta_2 + a_{12} delta_1 delta_2) / (b + b_1 delta_1 + b_2 delta_2 + b_{12} delta_1 delta_2).

## Key Parameters

- m (number of variable elements).
- 2^m (total coefficient count).
- Frequency s (the coefficients depend on s).

## When To Use

- Symbolic analysis in CAD.
- Insight into how each component affects the response.
- Crystal filter analysis where numerical cancellation requires symbolic forms.

## Risks & Pitfalls

- 2^m grows quickly; symbolic analysis is feasible only for small m (typically < 10).
- Some coefficients may be identically zero, simplifying the expression.

## Related Concepts

- [[concepts/symbolic-analysis]]
- [[concepts/large-change-sensitivity]]
- [[concepts/network-function]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
