---
title: "Carrier Mobility"
type: concept
tags: [semiconductor, device-physics, transport, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Definition

Carrier mobility mu is the proportionality between drift velocity and electric field in the low-field linear regime: v_d = mu E (units cm^2/V-s). It quantifies how readily carriers respond to an applied field and is set by the rate of momentum-relaxing scattering events.

## How It Works

mu = q tau_m / m*, where tau_m is the momentum-relaxation mean free time and m* the conductivity effective mass. Scattering mechanisms include acoustic-phonon scattering (mu ~ T^-3/2), ionized-impurity scattering (mu ~ T^3/2 / N_I), polar-optical-phonon scattering (important in III-V), intervalley scattering, and surface/interface scattering. Multiple mechanisms combine by Matthiessen's rule. At high field, mu becomes field-dependent and v_d saturates.

## Key Parameters

- Electron and hole mobilities mu_n, mu_p (e.g., Si: 1450, 450 cm^2/V-s at 300 K; GaAs: 8500, 400).
- Temperature dependence.
- Doping dependence (drops at high N_I).
- Field dependence (Caughey-Thomas-like empirical fits).

## When To Use

- Computing resistivity rho = 1 / (q mu_n n + q mu_p p).
- Estimating transconductance, drain current, and frequency response of FETs.
- Selecting material for high-speed devices (GaAs > Si).

## Risks & Pitfalls

- Hall mobility differs from drift/conductivity mobility by the Hall factor r_H (~1.18 phonon, ~1.93 ionized impurity).
- Surface mobility (e.g., in a MOSFET inversion layer) is lower than bulk because of additional surface scattering.

## Related Concepts

- [[concepts/effective-mass]]
- [[concepts/drift-diffusion-equation]]
- [[concepts/hall-effect]]
- [[concepts/einstein-relation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-14-chapter-10-transferred-electron-and-real-space-transfer-devices]]
- [[summaries/sze-physics-semiconductor-devices-21-appendix-e-properties-of-important-semiconductors]]
