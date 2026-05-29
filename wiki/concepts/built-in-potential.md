---
title: Built-in Potential
type: claim
id: concepts/built-in-potential
tags:
- semiconductor
- device-physics
- p-n-junction
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The built-in potential V_bi is the equilibrium electrostatic potential difference across a p-n junction (or other semiconductor junction), arising from carrier diffusion that creates space charge near the interface. For a nondegenerate p-n junction V_bi = (kT/q) ln(N_A N_D / n_i^2).

## How It Works

At equilibrium the Fermi level is flat throughout. The asymmetry between p-side and n-side Fermi-level positions inside their respective gaps must be made up by a position-dependent band bending across the depletion region: this band bending equals q V_bi. At room temperature in Si, V_bi ~ 0.7-0.9 V for typical doping; in GaAs ~ 1.2-1.4 V.

## Key Parameters

- Doping on both sides (N_A, N_D).
- Temperature T (via kT/q and via n_i).
- Bandgap (sets n_i; wider gap implies larger V_bi for the same doping).

## When To Use

- Computing depletion widths under any bias V: total drop = V_bi - V.
- Extracting doping from 1/C^2 vs V intercept.
- Predicting cut-in voltage of a diode.

## Risks & Pitfalls

- For degenerate doping, simple Boltzmann formula fails; the difference between Fermi levels in the bulk regions must be used directly.
- Bandgap narrowing at heavy doping reduces V_bi.

## Related Concepts

- [[concepts/depletion-region]]
- [[concepts/p-n-junction]]
- [[concepts/junction-capacitance]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
