---
title: "Stiff Systems"
type: concept
tags: [transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"]
confidence: high
---

## Definition

A stiff system is a system of ODEs (typically stable) whose Jacobian has eigenvalues with widely separated magnitudes — some poles close to the origin and some very far from it, all in the left half-plane. The fast-decaying components require small step sizes for explicit stability even after they have essentially vanished.

## How It Works

Circuit examples: a circuit with both a fast transistor switching transient (ns) and a slow filter response (ms). A non-stiff integrator like forward Euler must use h small enough to keep h * (largest |lambda|) inside its tiny stability region — wasteful when the fast component is irrelevant after the first few time units.

A-stable or stiffly-stable implicit methods (backward Euler, trapezoidal, BDF/Gear) overcome this: h can be chosen based on accuracy requirements for the slow component, regardless of the fast one.

## Key Parameters

- Stiffness ratio = (largest |lambda|) / (smallest |Re lambda|).
- Step-size selection criterion (accuracy vs. stability).
- Choice of integrator (stiffly-stable required for stiff systems).

## When To Use

- Circuit simulation with widely varying time constants (essentially always for IC simulation).
- Chemical kinetics, control systems, and biological models.
- Whenever explicit methods take prohibitively many steps.

## Risks & Pitfalls

- Misidentifying a system as stiff or non-stiff leads to wrong solver choice.
- Stiffly stable methods often have numerical damping; check whether oscillation modes need to be preserved.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/backward-euler]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
