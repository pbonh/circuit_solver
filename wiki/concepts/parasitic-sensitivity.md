---
title: Parasitic Sensitivity
type: claim
id: concepts/parasitic-sensitivity
tags:
- sensitivity
- analog
- well-established
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

A parasitic is an element with nominal value zero in the ideal circuit (e.g., a stray capacitance, layout inductance, leakage conductance). Although such an element has no effect on the nominal response, a small increment in its value can produce a measurable response variation. Sensitivity to parasitics uses the semi-normalized form S_h^tilde = (1/F)(dF/dh) since normalized sensitivity is identically zero when h = 0.

## How It Works

The total-variation formula becomes:

delta F / F ≈ sum_{nonzero h_i} S_{h_i}^F (delta h_i / h_i) + sum_{zero h_j} S_{h_j}^tilde (delta h_j),

where the second sum accounts for parasitics whose absolute value (not relative) is bounded by the fabrication process. Once layout is fixed, parasitics' delta h_j cannot be reduced; the only freedom is to design with small S_{h_j}^tilde at the nominal operating point.

## Key Parameters

- Magnitude of parasitic (controlled by layout / process).
- Position in the circuit (which sensitivity it contributes to).
- Frequency (parasitic capacitances matter most at high frequency).

## When To Use

- Layout-aware analog design.
- Robust design where component layout uncertainty must be tolerated.
- Analysis of stray effects in integrated circuits.

## Risks & Pitfalls

- Many small parasitic contributions can sum to a large total.
- Mutual coupling among parasitics may be neglected in simple analyses.
- Linear sensitivity is valid only for small perturbations; large parasitics need re-analysis.

## Related Concepts

- [[concepts/semi-normalized-sensitivity]]
- [[concepts/sensitivity-analysis]]
- [[concepts/gain-sensitivity-product]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
