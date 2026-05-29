---
title: Hot-Carrier Effects
type: claim
id: claim-hot-carrier-effects
tags:
- semiconductor
- device-physics
- mosfet
- reliability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.65
---

## Definition

Hot-carrier effects are the population, generation, and damage processes associated with carriers that have gained energies well above the lattice temperature by acceleration in high electric fields. In MOSFETs, hot electrons near the drain can be injected into the gate oxide, creating interface states and oxide-trapped charge that drift Vt and degrade transconductance over time.

## How It Works

Near the drain end of an inverted channel, the lateral field is large and electrons reach kinetic energies exceeding the Si-SiO2 barrier (~3.1 eV). Some are injected into the oxide where they generate defects or get trapped. The threshold-voltage shift D Vt and transconductance degradation grow as a power law of stress time; the worst stress occurs at moderate V_GS (peak substrate current) rather than at V_GS = V_DS.

## Key Parameters

- Substrate current I_sub (proxy for hot-electron generation).
- Lateral field at the drain.
- Lightly-doped-drain (LDD) extension grading.
- Stress voltage and time.

## When To Use

- Setting voltage-derating rules for long-term operation.
- Designing LDD / extension implants and graded-junction structures.
- Reliability qualification at accelerated stress conditions.

## Risks & Pitfalls

- Hot-carrier degradation is often worse in nMOS than pMOS because of the lower electron barrier and higher mobility.
- Operating frequency, duty cycle, and self-heating modulate the apparent stress severity.

## Related Concepts

- [[concepts/short-channel-effects]]
- [[concepts/impact-ionization]]
- [[concepts/oxide-charge]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
