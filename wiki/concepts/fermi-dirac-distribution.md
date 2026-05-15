---
title: "Fermi-Dirac Distribution"
type: concept
tags: [semiconductor, device-physics, statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Definition

The Fermi-Dirac distribution F(E) = 1 / (1 + exp[(E - E_F)/kT]) gives the probability that a single-particle quantum state of energy E is occupied by an electron, for a system of fermions in thermal equilibrium at temperature T and chemical potential (Fermi level) E_F.

## How It Works

Integrating N(E) F(E) over the conduction band yields the electron density n; integrating N(E)[1 - F(E)] over the valence band yields the hole density p. For E_F well below E_c (nondegenerate semiconductors), F(E) is well approximated by the Boltzmann exponential and the integral collapses to simple Nc exp(-(Ec-EF)/kT). For degenerate semiconductors the full Fermi-Dirac integral F_{1/2} must be used.

## Key Parameters

- Fermi level E_F (set by doping and charge neutrality).
- Temperature T (sharpness of the cutoff is kT-wide).
- Density of states N(E).

## When To Use

- Computing equilibrium electron and hole densities.
- Determining ionization fractions of donor and acceptor impurities.
- Calculating quasi-Fermi levels under nonequilibrium injection.

## Risks & Pitfalls

- Boltzmann approximation breaks down when carrier density approaches Nc or Nv (heavy doping or low T).
- Bandgap narrowing at heavy doping must be accounted for in degenerate semiconductors.

## Related Concepts

- [[concepts/carrier-concentration]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/energy-band-structure]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
