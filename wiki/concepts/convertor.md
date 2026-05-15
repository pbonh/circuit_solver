---
title: "Convertor (Impedance Convertor)"
type: concept
tags: [foundational, analog, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: medium
---

## Definition

A convertor is a two-port defined by V1 = k1 V2, I1 = -k2 I2. The ideal transformer is the special case k1 = n, k2 = 1/n. If k1, k2 have the same sign the element is a positive impedance convertor (PIC); if opposite signs, a negative impedance convertor (NIC).

## How It Works

A NIC with appropriate k1, k2 makes the input impedance equal to the negative of the load impedance: Z_in = -Z_L (Problem P.1.9 in Chapter 1). This non-passive behavior is realizable only with active elements (op-amps).

## Key Parameters

- k1, k2 (sign and magnitude).
- PIC versus NIC distinction.

## When To Use

- Synthesizing negative impedances for active filter design.
- Generalizing the ideal transformer to arbitrary scaling.

## Risks & Pitfalls

- Negative-impedance behavior can cause oscillations; stability analysis is required.
- Real implementations have finite gain-bandwidth and noise.

## Related Concepts

- [[concepts/ideal-transformer]]
- [[concepts/gyrator]]
- [[concepts/operational-amplifier]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
