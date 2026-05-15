---
title: "Impedance and Admittance"
type: concept
tags: [foundational, analog, ac, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

In the Laplace domain, the impedance Z(s) = V/I and the admittance Y(s) = I/V = 1/Z(s) characterize a two-terminal element. For the standard linear time-invariant elements: Z_R = R, Z_C = 1/(sC), Z_L = sL; Y_R = G, Y_C = sC, Y_L = 1/(sL).

## How It Works

Setting s = j omega converts impedance/admittance into a complex-valued frequency response. The total impedance of series elements is the sum; the total admittance of parallel elements is the sum. These rules underlie nodal/MNA formulation.

## Key Parameters

- Element value (R, L, C).
- Complex frequency s = sigma + j omega.
- For sinusoidal steady state: replace s by j omega.

## When To Use

- AC and frequency-domain network analysis.
- Formulating nodal admittance matrices.
- Computing network functions, poles, and zeros.

## Risks & Pitfalls

- Impedance is undefined for an ideal current source (infinite); admittance is undefined for an ideal voltage source.
- Frequency-domain analysis ignores initial conditions, which must be re-introduced for transient response.

## Related Concepts

- [[concepts/resistor]]
- [[concepts/capacitor]]
- [[concepts/inductor]]
- [[concepts/laplace-transform]]
- [[concepts/network-function]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
