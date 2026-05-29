---
title: Dielectric Breakdown
type: claim
id: claim-dielectric-breakdown
tags:
- semiconductor
- device-physics
- mosfet
- reliability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
confidence:
  base: 0.65
---

## Definition

Dielectric breakdown is the catastrophic, irreversible failure of an insulating film under sustained electric stress. In MOS technology, time-dependent dielectric breakdown (TDDB) of the gate oxide is the dominant wear-out mechanism that sets the maximum operating voltage and stress lifetime of MOSFET gate stacks.

## How It Works

Electron tunneling under bias creates defects in the oxide; once a percolation path of defects spans the dielectric, a conductive filament forms and the dielectric loses its insulating property. Lifetime t_BD follows a Weibull distribution and depends exponentially on field and oxide thickness (E-model, 1/E-model, power-law model are competing extrapolations). The Eyring-form temperature acceleration is well-established.

## Key Parameters

- Oxide field E and thickness t_ox.
- Defect generation rate.
- Weibull shape factor beta and characteristic time tau_63.
- Temperature.

## When To Use

- Setting V_DD scaling and reliability budgets at each technology node.
- Comparing alternative dielectrics (SiO2 vs. SiON vs. HfO2 high-k).

## Risks & Pitfalls

- Statistical extrapolation from accelerated tests to use conditions can be model-dependent (factor-of-10 spread).
- Soft breakdown ("SBD") precedes hard breakdown and can be tolerated in some applications but not others.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/fowler-nordheim-tunneling]]
- [[concepts/oxide-charge]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
