---
title: Continuity Equation (Semiconductor)
type: claim
id: concepts/continuity-equation
tags:
- semiconductor
- device-physics
- transport
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The semiconductor continuity equations express conservation of electrons and holes: dn/dt = G_n - U_n + (1/q) div J_n and dp/dt = G_p - U_p - (1/q) div J_p, where G is the generation rate, U the net recombination rate, and J the current density. They couple to Poisson's equation and the drift-diffusion current densities to form the full set of equations of semiconductor device physics.

## How It Works

The equations state that the local rate of change of carrier density equals the net carriers injected by current flow minus the net carriers lost to recombination plus those gained from generation. Solving them under appropriate boundary conditions yields minority-carrier profiles (e.g., the famous exp(-x/L) profile in long-base diodes) and time-dependent responses (storage delay, transit time).

## Key Parameters

- Generation rate G (optical, impact ionization, thermal).
- Net recombination rate U (SRH, radiative, Auger).
- Mobility and diffusion coefficient (inside J).
- Boundary conditions (junction injection, surface recombination velocity).

## When To Use

- All time-dependent and steady-state device modeling beyond the simplest equilibrium analysis.
- Computing minority-carrier lifetimes, diffusion lengths, photo-generated currents.

## Risks & Pitfalls

- For very short channels, semiclassical continuity equations miss quantum corrections.
- Numerical stability requires care; coupled nonlinear equations are typically solved by Newton iteration.

## Related Concepts

- [[concepts/drift-diffusion-equation]]
- [[concepts/poisson-equation]]
- [[concepts/carrier-lifetime]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
