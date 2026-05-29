---
title: Shockley Diode Equation
type: claim
id: claim-shockley-diode-equation
tags:
- semiconductor
- device-physics
- p-n-junction
- device-model
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.85
---

## Definition

The Shockley diode equation J = J_s [exp(qV/(n kT)) - 1] gives the ideal current-voltage characteristic of a p-n junction. The saturation current J_s = q D_p p_no / L_p + q D_n n_po / L_n captures minority-carrier diffusion across the quasi-neutral regions; the ideality factor n is 1 for ideal injection and approaches 2 when generation-recombination in the depletion region dominates.

## How It Works

Forward bias lowers the junction barrier, exponentially enhancing minority-carrier injection. Steady-state diffusion through the field-free quasi-neutral regions (length comparable to L = sqrt(D tau)) sets the current. At reverse bias the exponential collapses and J -> -J_s. At large forward bias, high-injection roll-off (n -> 2) and series resistance bend the I-V curve.

## Key Parameters

- Saturation current J_s.
- Ideality factor n (1 for diffusion, 2 for recombination, may exceed 2 in tunneling).
- Series resistance R_s.
- Minority-carrier diffusion lengths L_p, L_n and lifetimes.

## When To Use

- Compact device modeling (SPICE diode and BJT models all use Shockley-style exponentials).
- Bandgap-reference and log-amplifier design (where exact exponential I-V is exploited).

## Risks & Pitfalls

- Real diodes have multiple distinct regions (n=2 at low V, n=1 at moderate V, high-injection roll-off, series-R limit).
- Self-heating modifies J_s and effective temperature.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/shockley-read-hall-recombination]]
- [[concepts/drift-diffusion-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
