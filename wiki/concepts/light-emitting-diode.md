---
title: Light-Emitting Diode (LED)
type: claim
id: claim-light-emitting-diode
tags:
- semiconductor
- device-physics
- photonic
- p-n-junction
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt
confidence:
  base: 0.65
---

## Definition

A light-emitting diode is a forward-biased p-n (typically heterojunction) diode in which the injected carriers recombine radiatively, emitting incoherent spontaneous-emission photons at energies near the bandgap. The narrow emission spectrum and wavelength are set by the bandgap of the active region.

## How It Works

Under forward bias, electrons and holes are injected into the junction; in a direct-bandgap material such as InGaN, GaAs, or AlGaInP, radiative band-to-band (or quantum-well) recombination dominates and photons are emitted in random directions. Light is extracted through transparent contacts, shaped chips, surface texturing, or epi-up encapsulation. White LEDs are typically blue or UV LEDs covered by phosphors that down-convert part of the spectrum.

## Key Parameters

- Internal quantum efficiency eta_int (radiative / total recombination).
- Extraction efficiency eta_ext.
- Wall-plug efficiency eta_wp (output power / electrical input).
- Luminous efficacy (lumens per electrical watt).
- Modulation bandwidth (set by carrier lifetime).

## When To Use

- Indicator lights, displays, illumination (solid-state lighting).
- Optocouplers and infrared remote controls.
- Short-range plastic-fiber communication.

## Risks & Pitfalls

- Efficiency droop at high current density.
- Phosphor aging in white LEDs.
- Color shift with temperature and drive current.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/heterojunction]]
- [[concepts/radiative-recombination]]
- [[concepts/semiconductor-laser]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
