---
title: Drift-Diffusion Equation
type: claim
id: claim-drift-diffusion-equation
tags:
- semiconductor
- device-physics
- transport
- device-model
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

The drift-diffusion equation expresses the current density in a semiconductor as the sum of a drift term (proportional to electric field) and a diffusion term (proportional to carrier-concentration gradient): J_n = q mu_n n E + q D_n dn/dx and J_p = q mu_p p E - q D_p dp/dx. Combined with Poisson's equation and the continuity equations, it is the workhorse model for compact and TCAD-level semiconductor device simulation.

## How It Works

The drift term arises from the response of carriers to the local electric field; the diffusion term arises from random thermal motion driving carriers down concentration gradients (Fick's law). The Einstein relation D = (kT/q) mu links the two coefficients in the nondegenerate regime. At high field, mobility becomes field-dependent and v_d saturates; for very short channels, drift-diffusion must be extended to energy-balance or full Boltzmann transport.

## Key Parameters

- Mobility mu(E, N, T).
- Diffusion coefficient D = (kT/q) mu (Einstein relation).
- Doping concentration and gradient.

## When To Use

- Steady-state and transient device simulation of diodes, BJTs, MOSFETs, solar cells.
- Compact models (SPICE BSIM, Gummel-Poon) derive their core equations from drift-diffusion.

## Risks & Pitfalls

- Breaks down in ballistic transport regime (channel length below mean free path).
- Does not capture velocity overshoot, hot-carrier effects, or quantum transport.
- Numerical solution requires care with Scharfetter-Gummel discretization to handle exponential carrier variations.

## Related Concepts

- [[concepts/carrier-mobility]]
- [[concepts/einstein-relation]]
- [[concepts/poisson-equation]]
- [[concepts/continuity-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
