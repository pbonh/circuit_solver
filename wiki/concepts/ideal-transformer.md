---
title: Ideal Transformer
type: claim
id: concepts/ideal-transformer
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

An ideal transformer is a two-port satisfying V1 = n V2, I1 = -I2/n, where n is the turns ratio. It transfers power instantaneously with no losses and no magnetizing inductance.

## How It Works

The constitutive equations are algebraic — no derivatives — so the ideal transformer is frequency-independent. It is a special case of the more general "convertor" (with k1 = n, k2 = 1/n). In MNA, the ideal transformer introduces one extra branch-current unknown and the two algebraic constraints.

## Key Parameters

- Turns ratio n.
- Sign convention (dot markings).

## When To Use

- Modeling power, audio, and isolation transformers in their ideal regime.
- Impedance scaling: load Z_L at port 2 appears as n^2 Z_L at port 1.

## Risks & Pitfalls

- Real transformers have magnetizing inductance, winding resistance, and core losses that the ideal model does not capture.
- An ideal transformer with a DC current source on one side produces an inconsistent equation.

## Related Concepts

- [[concepts/mutually-coupled-inductors]]
- [[concepts/convertor]]
- [[concepts/gyrator]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
