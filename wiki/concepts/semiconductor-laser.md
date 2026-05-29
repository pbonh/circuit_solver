---
title: Semiconductor Laser
type: claim
id: concepts/semiconductor-laser
tags:
- semiconductor
- device-physics
- photonic
- heterojunction
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A semiconductor laser (diode laser) is a heterojunction p-n diode whose active region operates under strong forward bias, achieving population inversion (carrier-density-dependent quasi-Fermi-level separation exceeding the photon energy), and is placed in an optical resonator that provides feedback and selects coherent lasing modes. The result is a narrow-spectrum, directional, coherent light source.

## How It Works

A double-heterostructure or quantum-well active region simultaneously confines carriers and optical mode. Above the threshold current density J_th, the round-trip gain in the cavity exceeds the round-trip loss, and spontaneous emission is amplified into coherent stimulated emission. Common cavity types: Fabry-Perot (cleaved facets), distributed-feedback (DFB), distributed Bragg reflector (DBR), and vertical-cavity (VCSEL).

## Key Parameters

- Threshold current density J_th (kA/cm^2 for bulk, hundreds of A/cm^2 for QW).
- Slope efficiency (output power per unit current above threshold).
- Spectrum linewidth and mode spacing.
- Modulation bandwidth (GHz to tens of GHz).
- Relative-intensity noise (RIN), wavelength tunability.

## When To Use

- Fiber-optic communication transmitters (1.3, 1.55 um).
- Optical-disk pickups (CD, DVD, Blu-ray).
- Laser printers, scanners, barcode readers.
- Solid-state pumping of fiber/crystal lasers.

## Risks & Pitfalls

- Facet damage at high power (catastrophic optical damage, COD).
- Mode hopping and chirp under modulation.
- Aging from dark-line / point-defect generation.

## Related Concepts

- [[concepts/light-emitting-diode]]
- [[concepts/heterojunction]]
- [[concepts/quantum-well]]
- [[concepts/population-inversion]]
- [[concepts/radiative-recombination]]
- [[concepts/vcsel]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
