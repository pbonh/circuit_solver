---
title: Poisson Equation
type: claim
id: claim-poisson-equation
tags:
- semiconductor
- device-physics
- electrostatics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
---

## Definition

Poisson's equation div(eps E) = rho (or equivalently d^2 psi/dx^2 = -rho / eps in 1-D, where psi is the electrostatic potential) relates the spatial divergence of the electric field to the local charge density. In semiconductor devices it is solved jointly with the carrier continuity and drift-diffusion equations to determine the self-consistent potential and carrier distribution.

## How It Works

The charge density is rho = q (p - n + N_D^+ - N_A^-). In the depletion approximation, free carriers are assumed absent and rho is set by the ionized dopants only, yielding analytical solutions (e.g., parabolic potential profile and depletion width in a step junction). Boundary conditions at material interfaces include continuity of D_normal (with sheet charges Q producing a jump) and continuity of psi.

## Key Parameters

- Dielectric permittivity eps_s.
- Charge density rho(x).
- Boundary conditions (applied bias, built-in potential, interface charges).

## When To Use

- Computing depletion widths and built-in potentials of p-n junctions and Schottky barriers.
- Solving MOS C-V curves and threshold voltages.
- Coupled with continuity equations in full TCAD device simulation.

## Risks & Pitfalls

- Depletion approximation fails near small bias and at the depletion edges.
- Quantum confinement near surfaces requires solving Schrodinger-Poisson self-consistently.

## Related Concepts

- [[concepts/drift-diffusion-equation]]
- [[concepts/continuity-equation]]
- [[concepts/p-n-junction]]
- [[concepts/mis-capacitor]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
