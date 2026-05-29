---
title: Carrier Concentration
type: claim
id: claim-carrier-concentration
tags:
- semiconductor
- device-physics
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

The carrier concentration is the volume density of mobile charge carriers in a semiconductor: electron density n (in cm^-3) in the conduction band and hole density p in the valence band. In thermal equilibrium and for nondegenerate doping, n p = ni^2 (mass-action law), where ni is the intrinsic carrier density.

## How It Works

n and p are obtained by integrating the density of states against the Fermi-Dirac (or Boltzmann) distribution. In a nondegenerate semiconductor: n = Nc exp(-(Ec - E_F)/kT) and p = Nv exp(-(E_F - Ev)/kT), with effective densities of states Nc, Nv determined by the density-of-state effective masses. Charge neutrality (n + N_A^- = p + N_D^+) ties E_F to net doping. Above ~100 K, shallow donors and acceptors are typically fully ionized.

## Key Parameters

- Effective densities of states Nc, Nv.
- Net doping N_D - N_A and ionization fractions.
- Temperature (controls ni and ionization).
- Bandgap Eg (sets ni = sqrt(Nc Nv) exp(-Eg/2kT)).

## When To Use

- Computing equilibrium operating point of any semiconductor device.
- Predicting depletion-region widths, junction potentials, and threshold voltages.
- Estimating intrinsic-temperature limits of devices (when ni approaches doping).

## Risks & Pitfalls

- At heavy doping, degeneracy and bandgap narrowing invalidate the simple Boltzmann formulas.
- Incomplete ionization can be important at low temperature (carrier freeze-out).

## Related Concepts

- [[concepts/fermi-dirac-distribution]]
- [[concepts/donor-acceptor-doping]]
- [[concepts/bandgap]]
- [[concepts/effective-mass]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-21-appendix-e-properties-of-important-semiconductors]]
