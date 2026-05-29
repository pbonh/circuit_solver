---
title: Donor and Acceptor Doping
type: claim
id: claim-donor-acceptor-doping
tags:
- semiconductor
- device-physics
- doping
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

Donor and acceptor doping is the intentional substitution of impurity atoms into a semiconductor lattice to create shallow energy levels that contribute mobile electrons (donors, n-type) or mobile holes (acceptors, p-type). Examples include P, As, Sb donors in Si and B, Al, Ga acceptors in Si.

## How It Works

A donor atom (e.g., P) has one more valence electron than the host (Si); the extra electron is loosely bound in a hydrogenic state with ionization energy ~ m_c* / (m_0 eps_s^2) * 13.6 eV ~ 25 meV (Si) - small enough that ionization is essentially complete near room temperature. Acceptors work analogously, capturing an electron and creating a hole. Deep impurities (e.g., Au in Si) have levels near mid-gap and act as recombination centers rather than as efficient carrier sources.

## Key Parameters

- Ionization energy (E_c - E_D for donors, E_A - E_v for acceptors).
- Donor/acceptor concentrations N_D, N_A.
- Ground-state degeneracy (g_D = 2, g_A = 4).
- Solid solubility limit in the host crystal.

## When To Use

- Designing junction profiles, channel doping, contact regions.
- Compensating unintentional background impurities.
- Introducing recombination centers (e.g., Au, Pt) to shorten minority-carrier lifetime for fast switching.

## Risks & Pitfalls

- Heavy doping causes bandgap narrowing and increased Auger recombination.
- Compensation reduces effective net carrier density without reducing scattering.
- Diffusion of dopants at high process temperatures can re-distribute the doping profile.

## Related Concepts

- [[concepts/carrier-concentration]]
- [[concepts/fermi-dirac-distribution]]
- [[concepts/p-n-junction]]
- [[concepts/shockley-read-hall-recombination]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
