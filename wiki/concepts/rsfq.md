---
title: "Rapid Single Flux Quantum (RSFQ)"
type: concept
tags: [superconductive, digital, emerging, vlsi]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt"]
confidence: low
---

## Definition

Rapid Single Flux Quantum (RSFQ) is the dominant superconductive digital logic family. Logic values are encoded as the presence or absence of a single flux quantum (Φ_0 = h/2e) pulse within a clock window. Most logic gates (AND, OR) are sequential (clocked) in RSFQ, in contrast with CMOS combinational gates.

## How It Works

RSFQ circuits are built from Josephson junctions and superconducting inductors. Pulses propagate along passive transmission lines (PTL, requiring impedance-matched drivers/receivers) or active Josephson transmission lines (JTL, requiring DC bias on every junction and offering controllable delay). Standard splitters fan out 2; higher-fanout splitters exist but degrade bias margins. Modern manufacturing supports ~6000 Josephson junctions per mm². An 8-bit superconductive microprocessor at 80 GHz has been demonstrated.

## Key Parameters

- Critical currents and inductances of Josephson loops.
- Operating temperature (~ 4 K).
- Pulse propagation speed on PTLs.
- Gate fanout.

## When To Use

- Ultra-high-frequency digital systems where cryogenic cooling is acceptable.
- Quantum computing control electronics.
- High-performance scientific instrumentation.

## Risks & Pitfalls

- Pulse-based logic mandates precise clock distribution.
- Reduced integration density vs CMOS.
- Limited fabrication ecosystem.

## Related Concepts

- [[concepts/single-flux-quantum]]
- [[concepts/josephson-junction]]
- [[concepts/passive-transmission-line]]
- [[concepts/josephson-transmission-line]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
