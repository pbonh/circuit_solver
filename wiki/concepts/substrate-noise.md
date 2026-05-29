---
title: Substrate Noise
type: claim
id: claim-substrate-noise
tags:
- vlsi
- mixed-signal
- analog
- signal-integrity
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/10-7-effective-resistance-of-finite-grids.txt
confidence:
  base: 0.65
---

## Definition

Substrate noise is unwanted signal coupling between circuits through the shared semiconductor substrate of an integrated circuit. In mixed-signal CMOS, current injected into the digital ground propagates through the substrate to disturb the analog ground (and analog signal references), degrading analog performance.

## How It Works

A digital switching circuit injects current into the substrate via its ground contacts; the substrate is modeled as a 3D resistive mesh. The induced analog-ground voltage is V_ga = I R_ga R_gd / (R_gd + R_s + R_ga) where R_s is the substrate resistance between digital and analog ground contacts. Larger contact separation increases R_s and reduces coupling, but only up to a point (~20 μm in the case study) beyond which separation no longer helps.

## Key Parameters

- Substrate resistivity and thickness.
- Analog and digital ground network resistances.
- Distance between substrate contacts.
- Switching current magnitude.

## When To Use

- Mixed-signal SoC floorplanning to determine guard-ring and substrate-contact placement.
- Sensitivity analysis of analog blocks to digital activity.

## Risks & Pitfalls

- Worst-case noise is hard to bound without detailed transient simulation.
- Silicon-on-insulator and guard-ring mitigations complicate fabrication and cost.

## Related Concepts

- [[concepts/infinity-mirror-technique]]
- [[concepts/lattice-graph]]
- [[concepts/power-distribution-network]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
