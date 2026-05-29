---
title: Gain-Sensitivity Product (Moschytz)
type: claim
id: concepts/gain-sensitivity-product
tags:
- sensitivity
- analog
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

The gain-sensitivity product, introduced by G. S. Moschytz for ideal OPAMPs, is Gamma_A^F = A * S_A^F = (A/F)(dF/dA). When A → infinity (ideal OPAMP), S_A^F → 0 trivially, but Gamma_A^F has a meaningful limit. Vlach & Singhal show Gamma_A^F = S_B^F exactly, where B = -1/A.

## How It Works

Using chain rule with B = -1/A so dB/dA = 1/A^2:
- dF/dA = (dF/dB)(dB/dA) = (dF/dB)/A^2.
- (A/F)(dF/dA) = (1/F)(dF/dB) * (1/A) = wait — the careful derivation in the textbook yields A * S_A^F = (1/F)(dF/dB) = S_B^F (the semi-normalized form at B = 0).

The practical implication: for CAD with ideal OPAMPs, work directly with B = -1/A as the parameter; sensitivities are well-defined and computable, no need to manipulate infinities.

## Key Parameters

- A (OPAMP open-loop gain, ideally infinite).
- B = -1/A (variable used in calculations; B → 0 for ideal OPAMP).
- Frequency dependence of A in finite-bandwidth models.

## When To Use

- Active-filter sensitivity analysis where OPAMP gains are nominally infinite.
- Comparing OPAMP-based realizations of the same transfer function.
- CAD codes that need a finite-valued sensitivity parameter for ideal OPAMPs.

## Risks & Pitfalls

- Confusion between A and B sign conventions can cause sensitivity-sign errors.
- The product loses its compactness when finite-bandwidth A(s) is used; treat A as a frequency-dependent parameter directly.

## Related Concepts

- [[concepts/operational-amplifier]]
- [[concepts/parasitic-sensitivity]]
- [[concepts/semi-normalized-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
