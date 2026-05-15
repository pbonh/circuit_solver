---
title: "Clock Phase Formulation (SC Networks)"
type: concept
tags: [switched-capacitor, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt"]
confidence: medium
---

## Definition

The clock phase formulation analyzes switched-capacitor networks by treating each clock phase as a distinct LTI network. Switches that are closed during a given phase are short circuits in the corresponding LTI network; open switches are open circuits. Charge conservation at the boundaries between phases couples the phase equations.

## How It Works

For two-phase SC networks (phase 1 and phase 2):
1. Build the LTI nodal/MNA matrix for phase 1 (with certain switches closed).
2. Build the LTI nodal/MNA matrix for phase 2 (with the complementary switches closed).
3. Apply charge-conservation boundary conditions at each phase transition.
4. Solve the coupled per-phase systems.

Two-graph modified-nodal formulation (Chapter 4) is particularly compact for SC networks because switch states map naturally to graph edge collapse/deletion.

## Key Parameters

- Number of clock phases.
- Per-phase network topology.
- Boundary-condition handling (charge or voltage continuity).

## When To Use

- Analysis of any switched-capacitor analog circuit.
- SC filter design and simulation.

## Risks & Pitfalls

- Non-overlapping clocks must be modeled explicitly; assuming instantaneous phase transitions can hide real-circuit timing issues.
- Multi-phase clocks (>2 phases) complicate the formulation.

## Related Concepts

- [[concepts/switched-capacitor-network]]
- [[concepts/two-graph-modified-nodal]]
- [[concepts/switch-model]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
