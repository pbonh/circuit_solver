---
title: "Resistor"
type: concept
tags: [foundational, analog, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

A resistor is a two-terminal element relating the current i through it and the voltage v across it by Ohm's law v = R i = i/G, where R is the resistance in ohms and G the conductance in mhos. In an i-v graph, a linear time-invariant resistor is a straight line through the origin.

## How It Works

The constitutive equation v = R i applies in the time domain and unchanged in the Laplace domain. Short circuits correspond to R = 0 (characteristic on the i-axis); open circuits correspond to G = 0 (characteristic on the v-axis). Nonlinear resistors may be current-controlled (v unique in i, not vice versa) or voltage-controlled (i unique in v, not vice versa). Time-varying resistors are described by R(t).

## Key Parameters

- R (resistance, ohms) or equivalently G = 1/R (conductance, mhos).
- Linearity: linear if v = R i exactly; otherwise the i-v curve is curved.
- Time variance.

## When To Use

- As the fundamental building block of all networks.
- Modeling biases, source internal impedance, and load impedances.
- In numerical simulation, contributes a constant entry to the conductance matrix.

## Risks & Pitfalls

- Real components have parasitic inductance and capacitance not captured by ideal R.
- A "non-Ohmic" element whose i-v curve passes through the origin is not a resistor in the sense of Ohm's law (Vlach and Singhal point this out explicitly).

## Related Concepts

- [[concepts/capacitor]]
- [[concepts/inductor]]
- [[concepts/impedance-admittance]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
