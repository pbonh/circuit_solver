---
title: Reverse Recovery
type: claim
id: concepts/reverse-recovery
tags:
- semiconductor
- device-physics
- p-n-junction
- transient
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Reverse recovery is the transient response of a p-n junction diode when switched from forward conduction to reverse blocking. Because forward conduction stores minority-carrier charge in the quasi-neutral regions, the device cannot block reverse voltage instantaneously: a substantial reverse current flows during the storage time t_s, then decays during the fall time t_f. The total reverse-recovery time t_rr = t_s + t_f is a key switching parameter.

## How It Works

The forward current injects minority carriers whose density profile decays exponentially over the diffusion length. When the bias reverses, the stored charge supplies the reverse current at a rate set by the external circuit, until the carriers near the junction are extracted enough for a depletion region to re-form. After that, the depletion region grows and the reverse current decays to leakage levels.

## Key Parameters

- Minority-carrier lifetime tau.
- Forward current I_F and reverse current I_R.
- Junction grading.
- Lifetime control techniques (Au, Pt doping; electron irradiation).

## When To Use

- Designing fast-recovery diodes for switching power supplies, snubbers, freewheel diodes.
- Estimating dynamic losses in PWM converters.

## Risks & Pitfalls

- Snappy recovery generates EMI from rapid di/dt.
- Excessively low tau increases on-state voltage drop.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/carrier-lifetime]]
- [[concepts/junction-capacitance]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
