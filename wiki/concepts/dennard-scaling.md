---
title: Dennard Scaling
type: claim
id: concepts/dennard-scaling
tags:
- semiconductor
- device-physics
- mosfet
- scaling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Dennard (constant-field) scaling is the prescription that all linear MOSFET dimensions (L, W, t_ox, junction depth) and applied voltages be reduced by the same factor 1/k while doping is increased by k, so that internal electric fields remain constant. Under this rule, power density stays constant, delay improves linearly with 1/k, and integration density grows as k^2.

## How It Works

If V_DD, t_ox, L, W are each scaled by 1/k and N_a by k, then E ~ V/t_ox stays constant, depletion widths scale as 1/k, and the gradual-channel equations preserve all field magnitudes. Capacitance per area C_ox scales as k; capacitance per device C_ox W L scales as 1/k. Delay tau = C V / I scales as 1/k.

## Key Parameters

- Scaling factor k.
- Effective scaling rules (selective scaling: V_DD scaled less aggressively than L since the late 1990s).

## When To Use

- Setting technology-node targets for a CMOS process generation.
- Estimating power, density, and speed trends.

## Risks & Pitfalls

- Pure Dennard scaling broke down around the 90 nm node when leakage and reliability prevented V_DD scaling, and power density rose.
- New device structures (FinFET, gate-all-around) and materials (high-k, strain, channel-mobility boosters) are needed to continue scaling at modern nodes.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/short-channel-effects]]
- [[concepts/finfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
