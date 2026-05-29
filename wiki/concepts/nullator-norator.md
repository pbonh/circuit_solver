---
title: Nullator and Norator (Nullor)
type: claim
id: claim-nullator-norator
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
  base: 0.85
---

## Definition

A nullator is a two-terminal element defined by V = 0 and I = 0 simultaneously — neither realizable alone. A norator has no constitutive equation — both its voltage and current are arbitrary. A nullator paired with a norator forms a nullor, equivalent to an ideal operational amplifier or an ideal transistor.

## How It Works

A nullator forces the voltage and current at its port to be zero (matching the assumption of an op-amp's virtual short and zero input current). The norator absorbs the corresponding constraint — its voltage and current are determined by the rest of the network. The element pair imposes two equations and provides two unknowns, keeping the system balanced.

## Key Parameters

- Number of nullators must equal number of norators in a meaningful network.
- Placement: in op-amp modeling, the nullator goes across the differential inputs, the norator at the output.

## When To Use

- Preliminary analysis and design of op-amp and transistor circuits when ideal behavior suffices.
- Theoretical synthesis of two-port networks (active-filter design).
- Modeling tubes and ideal transistors (Fig. 1.6.6 in the chapter).

## Risks & Pitfalls

- Nullor-based models hide finite gain, bandwidth, and offset of real devices.
- Some simulators do not natively support nullators/norators and require an equivalent VVT model.

## Related Concepts

- [[concepts/operational-amplifier]]
- [[concepts/dependent-source]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
