---
title: Carrier Lifetime
type: claim
id: claim-carrier-lifetime
tags:
- semiconductor
- device-physics
- recombination
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
---

## Definition

The carrier lifetime tau is the characteristic time over which an excess minority-carrier population decays toward thermal equilibrium through recombination. For low-level injection in an n-type semiconductor, dpn/dt = -(pn - pn0)/tau_p.

## How It Works

The dominant mechanism determines tau: band-to-band radiative recombination (tau ~ 1/(R_ec N)) in direct-gap materials, Auger recombination (tau ~ 1/(C n^2)) in heavily injected or heavily doped regions, or SRH trap-assisted recombination (tau ~ 1/(sigma v_th N_t)) in indirect-gap materials. The diffusion length L = sqrt(D tau) is the practical decay length of excess minority carriers from a localized source. Measurement techniques include photoconductive decay (Stevenson-Keyes), photoelectromagnetic effect, and microwave-photoconductance decay.

## Key Parameters

- Mechanism-specific rate constants (R_ec, C_n, C_p, sigma_n, sigma_p, N_t).
- Doping and injection level.
- Temperature.
- Surface vs. bulk lifetime (and surface recombination velocity S).

## When To Use

- Predicting forward-bias diode currents and BJT current gain.
- Determining solar-cell collection efficiency (long tau improves performance).
- Tailoring fast switching by adding recombination centers (gold, platinum, irradiation).

## Risks & Pitfalls

- Lifetime is highly process-sensitive; cannot rely on textbook values.
- Generation lifetime in depleted regions can be much larger than recombination lifetime.
- High-level injection changes the effective tau.

## Related Concepts

- [[concepts/shockley-read-hall-recombination]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/continuity-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
