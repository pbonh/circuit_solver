---
title: p-n Junction
type: claim
id: claim-p-n-junction
tags:
- semiconductor
- device-physics
- p-n-junction
- diode
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt
confidence:
  base: 0.65
---

## Definition

A p-n junction is the interface between a p-type (acceptor-doped) and an n-type (donor-doped) region of the same or different semiconductor crystal. It is the most-fundamental building block of semiconductor devices and the basis for diodes, bipolar transistors, photodetectors, solar cells, LEDs, and many other devices.

## How It Works

At equilibrium, electron and hole diffusion across the junction sets up a depletion region of fixed ionized dopants that creates a built-in electric field opposing further diffusion. Forward bias narrows the depletion region and exponentially increases minority-carrier injection and current; reverse bias widens the region and supports a small saturation current until avalanche or Zener breakdown.

## Key Parameters

- Built-in potential V_bi, doping levels N_a, N_d.
- Depletion-region width and junction capacitance C_j(V).
- Saturation current I_s, ideality factor n.
- Breakdown voltage and series resistance.

## When To Use

- As a rectifier or signal diode.
- As a building block embedded inside larger devices (transistor junctions, photodiodes, varactors).
- For controlled charge storage (varicap tuning, charge pumps).

## Risks & Pitfalls

- High-injection effects modify the simple Shockley equation.
- Recombination in the depletion region causes non-ideal ideality factors near 2.
- Reverse-recovery transient stores minority-carrier charge.

## Related Concepts

- [[concepts/semiconductor-device]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/carrier-lifetime]]
- [[concepts/impact-ionization]]
- [[concepts/poisson-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-05-part-ii-device-building-blocks]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
- [[summaries/sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
