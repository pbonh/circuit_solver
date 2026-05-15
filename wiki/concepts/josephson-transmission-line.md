---
title: "Josephson Transmission Line (JTL)"
type: concept
tags: [superconductive, vlsi, interconnect, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt"]
confidence: low
---

## Definition

A Josephson transmission line (JTL) is an active superconductive interconnect built from a chain of Josephson junctions, each biased with a DC current and connected by superconducting inductors. JTLs propagate SFQ pulses with controllable delay and amplification.

## How It Works

Each junction in the chain switches in response to an incoming flux quantum, generating a fresh pulse for the next stage. The propagation delay per stage is determined by junction characteristics and bias current. By including JTLs as discrete delay elements, designers tune clock-path delays precisely without long PTL routes.

## Key Parameters

- Number of stages in the JTL.
- Bias current per junction.
- Delay per stage.
- Area per stage (dominant constraint on density).

## When To Use

- Variable-delay elements for clock tree skew tuning.
- Pulse regeneration over long distances.
- Local signal driving with amplification.

## Risks & Pitfalls

- Requires dedicated device-layer space.
- Bias-current routing adds layout complexity.

## Related Concepts

- [[concepts/passive-transmission-line]]
- [[concepts/josephson-junction]]
- [[concepts/rsfq]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
