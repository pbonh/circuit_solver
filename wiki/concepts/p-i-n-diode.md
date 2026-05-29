---
title: p-i-n Diode
type: claim
id: claim-p-i-n-diode
tags:
- semiconductor
- device-physics
- p-n-junction
- power-device
- rf
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.65
---

## Definition

A p-i-n diode is a three-region diode consisting of a heavily doped p-region, a wide intrinsic (or lightly doped) i-region, and a heavily doped n-region. The intrinsic region supports a large reverse voltage and provides a high-resistance microwave attenuator or switch when reverse-biased, while in forward bias it is flooded with carriers and behaves as a low-resistance.

## How It Works

Under reverse bias the i-region fully depletes (large W increases V_BR and lowers capacitance). Under forward bias, electrons and holes are injected from both heavily doped contacts; conductivity modulation reduces on-state resistance dramatically. At RF frequencies the diode behaves as a current-controlled resistor because minority-carrier lifetime exceeds the RF period.

## Key Parameters

- Intrinsic-region width W (sets V_BR and on-resistance).
- Minority-carrier lifetime tau in the i-region.
- Forward current (sets RF resistance).
- Reverse-bias capacitance (lower than a p-n junction of same V_BR).

## When To Use

- High-voltage rectifiers and power switches (silicon p-i-n).
- RF/microwave switches and variable attenuators.
- High-energy radiation detectors (large depleted active volume).

## Risks & Pitfalls

- Storage time during switching scales with i-region width and lifetime.
- At very high frequencies the conductivity modulation no longer responds; the diode behaves as a capacitor.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/depletion-region]]
- [[concepts/avalanche-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
