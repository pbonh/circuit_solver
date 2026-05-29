---
title: Gyrator
type: claim
id: claim-gyrator
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
  base: 0.85
---

## Definition

A gyrator is a two-port with the constitutive equations I1 = -g2 V2, I2 = g1 V1 (g1, g2 are gyration constants). When g1 = g2 = g, the gyrator is called ideal. Equivalently, V1 = r1 I2, V2 = -r2 I1 with r_i = 1/g_i.

## How It Works

A gyrator can be realized by two VCTs (one in each direction) or by two CVTs. Loaded by a capacitor at port 2, an ideal gyrator presents an inductive impedance at port 1: Z_in = g^2 / Z_load — a key reason it is used in active filters to synthesize inductors.

## Key Parameters

- Gyration conductances g1, g2 (or resistances r1, r2).
- Symmetry: g1 = g2 = g for the ideal gyrator.

## When To Use

- Synthesizing inductors from capacitors and active devices in IC design.
- Active filter realizations.

## Risks & Pitfalls

- Active gyrator realizations require op-amps and are subject to bandwidth and stability constraints.
- An ideal gyrator is non-reciprocal (unlike a transformer); sign conventions matter.

## Related Concepts

- [[concepts/ideal-transformer]]
- [[concepts/dependent-source]]
- [[concepts/operational-amplifier]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
