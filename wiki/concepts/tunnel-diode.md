---
title: "Tunnel Diode"
type: concept
tags: [semiconductor, device-physics, p-n-junction, tunneling, rf, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/12-chapter-8-tunnel-devices.txt"]
confidence: medium
---

## Definition

A tunnel (Esaki) diode is a heavily doped (degenerate on both sides) p-n junction with a depletion region narrow enough (typically < 10 nm) for direct band-to-band tunneling between conduction-band states on the n-side and valence-band states on the p-side. Its I-V curve exhibits a region of negative differential resistance between a peak voltage V_p and a valley voltage V_v.

## How It Works

At small forward bias, the n-side conduction-band electrons line up with empty valence-band states on the p-side and tunnel directly across; the current rises with V to a peak at V_p. As V grows further, the overlap of filled and empty states shrinks and the tunneling current falls -- yielding negative dI/dV. Beyond V_v, normal diffusion current takes over.

## Key Parameters

- Peak current I_p and valley current I_v.
- Peak-to-valley current ratio (PVCR), key figure of merit for logic.
- Peak voltage V_p, valley voltage V_v, valley-to-peak voltage ratio.
- Junction capacitance.
- Negative resistance R_n in the NDR region.

## When To Use

- Microwave oscillators and amplifiers (historical importance).
- Low-power voltage references and zero-bias detectors.
- Functional logic and memory cells with reduced transistor count.

## Risks & Pitfalls

- Two-terminal devices have no isolation between input and output -- requires careful circuit topology.
- Peak current and PVCR have process-induced spread.

## Related Concepts

- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/p-n-junction]]
- [[concepts/negative-differential-resistance]]
- [[concepts/zener-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
