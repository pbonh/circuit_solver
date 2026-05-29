---
title: Scattering Parameters (S Parameters)
type: claim
id: concepts/scattering-parameters
tags:
- analog
- rf
- well-established
- characterization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Scattering parameters (S parameters) characterize a linear multi-port network by the ratios b_m / a_k of outgoing to incoming normalized power waves at the ports, for stimulus applied at port k and response measured at port m. The full S-matrix relates all incoming a_k and outgoing b_m signals.

## How It Works

Each port has a reference impedance Z_k. Normalized power waves are a_k = (V_k + I_k Z_k) / (2√|Re(Z_k)|) and b_k = (V_k − I_k Z_k*) / (2√|Re(Z_k)|). The S-matrix B = S A is a function of frequency. Other representations (Z, Y, ABCD, h) can be derived from S. Crucially, S parameters require no knowledge of internal structure — they treat the network as a black box.

## Key Parameters

- Number of ports.
- Reference impedance Z_k (typically 50 Ω for RF).
- Frequency range and resolution.

## When To Use

- Characterization of RF and high-speed digital components.
- Vendor-supplied IP blocks where structure is intellectual property.
- Behavioral modeling of complex passive components (packages, connectors).

## Risks & Pitfalls

- Measurement errors compound when converting between representations.
- Causality and passivity must be enforced when fitting models.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
