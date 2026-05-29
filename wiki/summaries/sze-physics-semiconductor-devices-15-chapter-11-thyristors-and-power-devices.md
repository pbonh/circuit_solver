---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 11: Thyristors
  and Power Devices'
type: source
id: source-sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices
kind: derived-summary
tags:
- semiconductor
- device-physics
- power-device
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/15-chapter-11-thyristors-and-power-devices.txt
---

## Key Points

- Thyristors are p-n-p-n four-layer devices that latch between a high-impedance forward-blocking state and a low-impedance forward-conducting state, with reverse-blocking capability as well. They are workhorses for high-voltage, high-current line-frequency switching.
- Thyristor regenerative model: the four-layer structure can be viewed as two cross-coupled transistors (one pnp and one npn) whose loop gain alpha_pnp + alpha_npn = 1 marks the boundary between blocking and conducting. Below this, the device blocks; above, regenerative feedback drives it to saturation.
- Reverse blocking: relies on the depletion region of the reverse-biased outer junctions; breakdown voltage scales with the lightest doping.
- Forward blocking: depends on V_GS-controlled barrier; before turn-on, the center junction blocks while the outer junctions are forward-biased.
- Turn-on mechanisms: gate current injection (the canonical SCR), light injection (light-activated thyristor, LASCR/LCR), avalanche injection (dV/dt triggering as a parasitic mechanism), and temperature.
- Forward conduction: heavy carrier injection conductivity-modulates the wide drift region, giving low on-state voltage (1-2 V) at thousands of A/cm^2.
- Static I-V curves: distinct holding current, latching current, breakover voltage.
- Turn-on/off times: turn-on is fast (microseconds); turn-off in conventional SCRs requires external commutation because the internal carrier storage must decay (tens of microseconds to milliseconds).
- Thyristor variants: gate turn-off (GTO) thyristor can be turned off by negative gate current; diac and triac (bidirectional thyristors) for AC switching; light-activated thyristor for HVDC valve stacks.
- Power-device-only chapter section: IGBT combines MOSFET input with BJT output; static-induction transistor (SIT) is a JFET-like vertical device with no pinch-off, used at high voltages; static-induction thyristor is a hybrid.
- Specific power-device topics covered: edge termination (mesa, beveled, junction-termination extensions, guard rings), conductivity modulation, second breakdown, thermal limits, and safe operating area.

## Relevant Concepts

- [[concepts/thyristor]]
- [[concepts/igbt]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/mosfet]]
- [[concepts/p-n-junction]]
- [[concepts/avalanche-breakdown]]
- [[concepts/junction-termination]]

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 11 — Thyristors and Power Devices
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/15-chapter-11-thyristors-and-power-devices.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
