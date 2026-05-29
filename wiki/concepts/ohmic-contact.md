---
title: Ohmic Contact
type: claim
id: claim-ohmic-contact
tags:
- semiconductor
- device-physics
- contact
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt
confidence:
  base: 0.85
---

## Definition

An ohmic contact is a low-resistance metal-semiconductor (or other heterocontact) junction that exhibits a linear, symmetric current-voltage characteristic and a negligible voltage drop compared to the device on which it is mounted. Every semiconductor device requires ohmic contacts at its terminals to inject and extract carriers without unwanted rectification.

## How It Works

Ohmic behavior is achieved by heavily doping the semiconductor under the metal so that the depletion region becomes so thin that carriers can tunnel through it (field emission). The specific contact resistivity rho_c is the figure of merit, with rho_c ~ exp(2 sqrt(eps_s m*) phi_B / (hbar sqrt(N_D))), decreasing exponentially as N_D increases. Silicide formation (TiSi2, NiSi, PtSi, CoSi2) provides a low-resistance, thermally stable contact metallurgy in silicon technology; refractory metals and alloyed contacts (AuGe/Ni in GaAs) are used in III-V.

## Key Parameters

- Specific contact resistivity rho_c (Ohm-cm^2).
- Barrier height phi_B (lower is better).
- Surface doping N_D (higher is better).
- Thermal stability and electromigration resistance.

## When To Use

- Source/drain contacts of MOSFETs (NiSi, CoSi2 in modern Si CMOS).
- Emitter and collector contacts of BJTs.
- Anode/cathode contacts of LEDs, lasers, and photodetectors.

## Risks & Pitfalls

- Inadequate doping under the contact yields a parasitic Schottky barrier and high series resistance.
- Spiking and reaction between metal and semiconductor during anneal can short out shallow junctions.
- Electromigration at high current density limits long-term reliability.

## Related Concepts

- [[concepts/schottky-barrier]]
- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
