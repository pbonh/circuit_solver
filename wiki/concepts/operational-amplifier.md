---
title: Operational Amplifier (OPAMP)
type: claim
id: concepts/operational-amplifier
tags:
- foundational
- analog
- well-established
- device-model
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An operational amplifier is a high-gain differential amplifier widely used as a building block in analog signal processing. Vlach and Singhal define the ideal OPAMP via the nullor relations (V1 = 0, I1 = 0 at the differential input pair) — equivalent to infinite differential gain, zero input bias current, and zero output impedance.

## How It Works

In the ideal model, the input is a nullator (V = 0, I = 0) and the output is a norator (no constitutive equation). Commercial OPAMPs are constructed from transistors and approximate a VVT with very high but finite mu, finite bandwidth, and non-zero output impedance. The 741C linear model (Fig. 3 of the Motivation chapter) is used to illustrate realistic OPAMP characteristics elsewhere in the book.

## Key Parameters

- Differential gain mu (ideal: infinite; real: 10^5 to 10^6 at DC).
- Input bias current and offset voltage (ideal: zero).
- Output impedance (ideal: zero).
- Bandwidth (ideal: infinite; real: gain-bandwidth product is the figure of merit).
- Slew rate, common-mode rejection ratio (for realistic models).

## When To Use

- Active-filter synthesis and analog signal processing.
- Operational implementation of differentiators, integrators, summers.
- As a building block in any analog CAD library.

## Risks & Pitfalls

- The ideal-OPAMP model is too optimistic at high frequencies; finite bandwidth must be modeled for designs above several Hz.
- Stability requires careful frequency-compensation; closed-loop poles can shift dramatically with feedback.
- Output saturation introduces a hard nonlinearity not captured by linear models.

## Related Concepts

- [[concepts/nullator-norator]]
- [[concepts/dependent-source]]
- [[concepts/macromodeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
