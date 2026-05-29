---
title: Thyristor
type: claim
id: claim-thyristor
tags:
- semiconductor
- device-physics
- power-device
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/15-chapter-11-thyristors-and-power-devices.txt
confidence:
  base: 0.65
---

## Definition

Per Sze & Ng (Sect. 11.2): "The name *thyristor* applies to a general family of semiconductor devices that exhibit bistable characteristics and can be switched between a high-impedance, low-current off-state and a low-impedance, high-current on-state. Also, the operations of thyristors are intimately related to the bipolar-transistor action in which both electrons and holes interact with each other in the transport processes." The basic thyristor (Fig. 1a of the chapter) is a four-layer p-n-p-n device with three series p-n junctions J1, J2, J3; the n1-layer ("n-base") is much wider and lowest-doped to sustain high breakdown voltage. Anode contacts the outer p-layer; cathode the outer n-layer; the gate (also called the "base") is connected to the inner p-base. Without the gate the two-terminal device is the **Shockley diode**.

## How It Works

The historical analysis (Moll et al. 1956; control terminal added 1958 by Mackintosh and Aldrich-Holonyak) uses the two-transistor analogue: the p-n-p-n stack is equivalent to a cross-coupled p-n-p plus n-p-n pair. When the loop gain `α_pnp + α_npn` approaches 1, regenerative feedback latches the device on (region 1→2 of Fig. 2 in Sze Sect. 11.2: forward breakover from off-state). Re-blocking requires the main current to fall below the holding current (SCR/Shockley diode) or active forced commutation / MOS turn-off (GTO/MCT).

## Key Parameters

- Forward breakover voltage and reverse blocking voltage.
- Holding and latching currents.
- On-state voltage drop and surge current rating.
- di/dt and dv/dt limits.

## When To Use

- High-voltage, high-current line-frequency switching: phase control, AC motor drives, induction heating, HVDC valves.

## Risks & Pitfalls

- Loss of forward blocking from rapid dv/dt or surge-induced latch-up.
- Slow turn-off compared to MOSFETs/IGBTs; commutation circuitry adds complexity.
- Susceptible to parasitic latch-up in CMOS circuits where unintentional p-n-p-n structures exist.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/igbt]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
