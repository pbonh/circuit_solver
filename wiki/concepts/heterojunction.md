---
title: Heterojunction
type: claim
id: concepts/heterojunction
tags:
- semiconductor
- device-physics
- band-engineering
- photonic
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
  source_count: 2
  contradicted: false
  effective: 0.884
  inputs_hash: 48379ae317c01c10
---

## Definition

A heterojunction is a junction between two dissimilar semiconductors (typically with different bandgaps and electron affinities). Heterojunctions provide additional design freedom through conduction-band and valence-band offsets AE_c and AE_v that confine carriers and modify transport.

## How It Works

Band alignment falls into three classes: Type-I (straddling, AE_c and AE_v both push carriers toward the narrow-gap material), Type-II (staggered, electrons and holes accumulate on different sides), and Type-III (broken-gap, conduction band of one side lies below valence band of the other). When lattice constants are mismatched, thin strained-layer epitaxy can still grow coherently up to a critical thickness; beyond that, dislocations relieve the strain. Pairs like GaAs/AlGaAs and InGaAs/InP are workhorse systems.

## Key Parameters

- Band offsets AE_c, AE_v.
- Lattice mismatch (a_e - a_s)/a_s and critical thickness.
- Doping profiles on each side; modulation-doping for high-mobility 2DEG.

## When To Use

- Heterojunction bipolar transistors (HBTs) for high f_T.
- Modulation-doped FETs (MODFETs/HEMTs) for low-noise microwave amplifiers.
- Light-emitting diodes and laser diodes (carrier and optical confinement).
- Quantum wells and superlattices.

## Risks & Pitfalls

- Lattice mismatch beyond critical thickness creates defects and interface traps.
- Conduction- or valence-band spikes can impede current flow.
- Band-offset uncertainty complicates first-principles device design.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/quantum-well]]
- [[concepts/energy-band-structure]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
- [[summaries/sze-physics-semiconductor-devices-14-chapter-10-transferred-electron-and-real-space-transfer-devices]]
- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
