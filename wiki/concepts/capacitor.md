---
title: "Capacitor"
type: concept
tags: [foundational, analog, ac, transient, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

A capacitor is a two-terminal element relating charge q to voltage v by q = f(v). For a linear time-invariant capacitor, q = C v, giving the constitutive relations i = C dv/dt and v(t) = V0 + (1/C) ∫ i dτ. C is measured in farads.

## How It Works

In the Laplace domain, i(t) = C dv/dt becomes I = sCV - CV0, where V0 is the initial voltage. The initial-condition term can be modeled as an independent source: a current impulse of value CV0 in the admittance description, or a step source of value V0/s in the impedance description. The admittance of a capacitor is Yc = sC; its impedance is Zc = 1/(sC).

## Key Parameters

- C (capacitance, farads).
- Initial voltage V0.
- Linearity (q = f(v) in general; q = Cv for linear time-invariant).

## When To Use

- Modeling charge storage in linear and nonlinear devices.
- Frequency-dependent filtering and signal coupling.
- Representing parasitic and junction capacitances in semiconductor models.

## Risks & Pitfalls

- An ideal capacitor enforces voltage continuity (a step current produces a Dirac voltage), which can cause stiffness in numerical integration.
- Real capacitors have ESR and ESL not captured by an ideal model.
- In MNA, a series capacitor between two nodes contributes both to admittance and to algebraic-differential equations during transient analysis.

## Related Concepts

- [[concepts/resistor]]
- [[concepts/inductor]]
- [[concepts/impedance-admittance]]
- [[concepts/laplace-transform]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
