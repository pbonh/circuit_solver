---
title: Quantum-Mechanical Tunneling
type: claim
id: concepts/quantum-mechanical-tunneling
tags:
- semiconductor
- device-physics
- transport
- quantum
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Quantum-mechanical tunneling is the passage of a quantum particle through a classically forbidden potential barrier with nonzero probability. In semiconductors it underlies tunnel diodes, resonant-tunneling diodes, gate-oxide leakage, Zener breakdown, and floating-gate program/erase mechanisms.

## How It Works

The Schrodinger equation gives an exponentially decaying wavefunction inside a barrier; the transmission probability for a rectangular barrier of height U_0 and width W is approximately T ~ exp(-2 W sqrt(2 m* (U_0 - E)) / hbar). For arbitrary barrier shapes the WKB approximation gives T ~ exp(-2 integral of k(x) dx). The total tunneling current is then the product of the transmission probability with available carrier and empty-state densities on either side, integrated over energy.

## Key Parameters

- Barrier height (U_0 - E) and width W.
- Effective mass m* in the barrier.
- Available states on both sides (Fermi-Dirac distributions, density of states).

## When To Use

- Designing tunnel diodes and resonant-tunneling structures.
- Modeling gate-oxide leakage (direct, Fowler-Nordheim).
- Calculating Zener and band-to-band tunneling currents in heavily doped junctions.

## Risks & Pitfalls

- WKB ignores reflections at boundaries and underestimates current at thin barriers.
- For two-band or multiband problems, full transfer-matrix or NEGF methods are needed.
- Tunnel currents have extreme sensitivity to barrier thickness (exponential).

## Related Concepts

- [[concepts/thermionic-emission]]
- [[concepts/schottky-barrier]]
- [[concepts/effective-mass]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
