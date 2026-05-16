---
title: "Josephson Junction"
type: concept
tags: [superconductive, device, emerging, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt"]
confidence: medium
---

> GraphsInVLSI Chapter 11 treats Josephson junctions (JJ) operationally — as the device whose switching emits an SFQ pulse — rather than deriving the device physics. The chapter cites Likharev & Semenov [587] for the fundamental RSFQ theory and reports current manufacturing density of "over 6,000 Josephson junctions (JJ) per mm² [582]". Each junction in an active Josephson transmission line "[requires] bias current" to operate as a relay element.

## Definition

A Josephson junction is a quantum-mechanical device consisting of two superconducting electrodes separated by a thin insulating barrier (or weak link). It exhibits the Josephson effect: a supercurrent can tunnel through the barrier with no voltage drop until a critical current is exceeded, at which point a voltage pulse (and thus a flux quantum) is produced.

## How It Works

The supercurrent through a Josephson junction is I = I_c sin(φ), where φ is the phase difference of the superconducting wavefunctions. When the junction switches, it produces a quantized voltage pulse with integral ∫ V dt = Φ_0 = h/2e ≈ 2.07 mV·ps. This pulse is the fundamental information carrier in SFQ/RSFQ logic.

## Key Parameters

- Critical current I_c.
- Capacitance.
- Subgap resistance and McCumber damping parameter.

## When To Use

- Building blocks of all SFQ and RSFQ logic.
- Quantum computing qubits (transmon, flux qubits).
- High-sensitivity magnetometers (SQUIDs).

## Risks & Pitfalls

- Fabrication uniformity is challenging at scale.
- Cryogenic operation is required.

## Related Concepts

- [[concepts/single-flux-quantum]]
- [[concepts/rsfq]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
