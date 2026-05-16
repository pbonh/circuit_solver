---
title: "Insulated-Gate Bipolar Transistor (IGBT)"
type: concept
tags: [semiconductor, device-physics, power-device, mosfet, bjt, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/00-preface.txt", "raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/15-chapter-11-thyristors-and-power-devices.txt"]
confidence: medium
---

## Definition

Per Sze & Ng (Sect. 11.4.1): "The name *insulated-gate bipolar transistor* (IGBT) comes from its operation based on an internal interaction between an insulated-gate FET (IGFET) and a bipolar transistor." The device was first demonstrated by Baliga in 1979 and has been called IGT, IGR, and COMFET. Sze describes the structure (Fig. 27) two ways: "an SCR with a cathode short and an MOSFET (or more specifically, a DMOS transistor; see Section 6.5.6) connecting the n⁺-cathode to the n⁻-base" or equivalently "a DMOS transistor with an additional p-n junction within the drain region."

## How It Works

The bulk of the device is the n⁻-layer, which is the drain of the DMOS transistor and the base of the p-n-p bipolar transistor. It is "lightly doped and is wide in order to support a large blocking voltage. In the on-state, conductivity in this region is enhanced by excess electrons injected from the n⁺-cathode via the DMOS transistor surface channel, and by excess holes from the p⁺-anode. This conductivity modula[tion]" keeps the on-state voltage low. Turn-off is slower than a pure MOSFET because the injected minority-carrier charge must be removed.

The vertical IGBT (Fig. 27a) uses the p⁺-anode as a low-resistivity substrate with a ~50 µm epitaxial n⁻-layer doped below 10¹⁴ cm⁻³; isolation is difficult so devices are diced as discrete components. The lateral LIGT variant (Fig. 27b) achieves isolation through the p-type substrate.

## Key Parameters

- Breakdown voltage and on-state voltage drop V_CE(sat).
- Switching times and turn-off tail current.
- Safe operating area (SOA) for hard switching.

## When To Use

- Medium-to-high voltage, medium-frequency power switching: motor drives, induction heating, power supplies, traction inverters.

## Risks & Pitfalls

- Switching losses from minority-carrier storage limit frequency.
- Latch-up of the parasitic thyristor under fault conditions.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/thyristor]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-00-preface]]
- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
