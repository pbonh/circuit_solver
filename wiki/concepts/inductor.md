---
title: "Inductor"
type: concept
tags: [foundational, analog, ac, transient, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

An inductor is a two-terminal element relating magnetic flux phi to current i by phi = f(i). For a linear time-invariant inductor, phi = L i, giving v = L di/dt and i(t) = I0 + (1/L) ∫ v dτ. L is measured in henrys.

## How It Works

In the Laplace domain v = L di/dt becomes V = sLI - LI0. The initial-condition term I0 is modeled as a voltage impulse LI0 in series in the impedance description, or as a step current I0/s in parallel in the admittance description. The impedance of an inductor is ZL = sL; its admittance is YL = 1/(sL).

## Key Parameters

- L (inductance, henrys).
- Initial current I0.
- Mutual inductance M (when coupled to other inductors).

## When To Use

- Modeling magnetic energy storage.
- Filter design (LC tanks, ladder filters).
- Modeling parasitic lead inductances.

## Risks & Pitfalls

- An ideal inductor enforces current continuity; a step voltage produces a Dirac current — care is required in MNA stamping.
- In MNA, inductors introduce extra branch-current unknowns and algebraic-differential equations.
- Real inductors have winding resistance, distributed capacitance, and core losses.

## Related Concepts

- [[concepts/capacitor]]
- [[concepts/resistor]]
- [[concepts/mutually-coupled-inductors]]
- [[concepts/laplace-transform]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
