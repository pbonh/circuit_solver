---
title: Passive Transmission Line (PTL)
type: claim
id: concepts/passive-transmission-line
tags:
- superconductive
- vlsi
- interconnect
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

GraphsInVLSI Chapter 11: one of two RSFQ interconnect types — "a *passive transmission line* (PTL) requiring a driver, receiver, and impedance matching [589, 590]". Used to propagate single flux quantum (SFQ) pulses between RSFQ gates. The chapter contrasts PTL with the active [[concepts/josephson-transmission-line]] (JTL), which uses biased Josephson junctions instead of impedance-matched drivers/receivers.

## How It Works

The PTL behaves as a low-loss superconducting transmission line. SFQ pulses propagate at a velocity determined by the inductance and capacitance per unit length (typically a few μm/ps in modern fabrication). Propagation delay is linear in length, simplifying delay control via wire snaking.

## Key Parameters

- Characteristic impedance.
- Propagation velocity (typical ~6.25 μm/ps in M2/M3 layers, per case study).
- Driver/receiver delay.
- Maximum length (signal integrity limit).

## When To Use

- Long-distance interconnect between RSFQ gates.
- Reduced-loss alternative to Josephson transmission lines for static routing.

## Risks & Pitfalls

- Driver and receiver cells add fixed overhead.
- Impedance mismatch causes reflections.

## Related Concepts

- [[concepts/josephson-transmission-line]]
- [[concepts/rsfq]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
