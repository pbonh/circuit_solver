---
title: Dependent Sources (Transducers)
type: claim
id: concepts/dependent-source
tags:
- foundational
- analog
- well-established
- device-model
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A dependent (controlled) source is an ideal voltage or current source whose value depends on a voltage or current elsewhere in the network. Vlach and Singhal call them "transducers" and identify four types: VVT (voltage-to-voltage), VCT (voltage-to-current), CVT (current-to-voltage), CCT (current-to-current).

## How It Works

Each transducer is described by a two-port relation of the form M v + N i = e, with M, N matrices and e an external source vector. Specifically:
- VVT: I1 = 0; V2 = mu V1 (mu = voltage gain).
- VCT: I1 = 0; I2 = g V1 (g = transconductance).
- CVT: V1 = 0; V2 = r I1 (r = transresistance).
- CCT: V1 = 0; I2 = alpha I1 (alpha = current gain).

These elements stamp into MNA matrices with non-zero off-diagonal entries that may be asymmetric.

## Key Parameters

- Gain (mu, g, r, or alpha) for each type.
- Linearity and frequency dependence (gain may be a function of s for non-ideal sources).
- Frequency response of practical transducers (real op-amps are VVTs with finite bandwidth).

## When To Use

- Macromodeling transistors and op-amps in their linear regions.
- Building gyrators, convertors, and other ideal two-ports from primitive transducers.
- Representing controlled-source elements in SPICE-style netlists.

## Risks & Pitfalls

- Ideal transducers may produce indefinite MNA matrices.
- Direction-of-flow and polarity conventions must match the symbol; sign errors are common.
- A VVT with infinite mu approaches a nullor; numerical issues arise as mu grows large.

## Related Concepts

- [[concepts/operational-amplifier]]
- [[concepts/gyrator]]
- [[concepts/nullator-norator]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-11-7-graph-pair-decision-diagram]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
