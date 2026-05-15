---
title: "Normal Tree"
type: concept
tags: [graph, analog, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt"]
confidence: high
---

## Definition

A normal tree is a tree chosen specifically for state-variable formulation according to a priority rule that ensures the chosen state variables (twig-capacitor voltages, chord-inductor currents) are independent. Voltage sources are forced into the tree (their voltages are specified); current sources are forced into the cotree (their currents are specified).

## How It Works

Selection rule (Vlach & Singhal Section 3.9):
1. Place all independent voltage sources into the tree.
2. Add as many capacitors as possible to the tree.
3. Add as many resistors / conductors as possible to the tree.
4. Complete the tree by adding inductors as needed.

Then number twigs first and chords last, with sources numbered separately. The state vector X comprises tree capacitor voltages and chord inductor currents — variables corresponding to independent energy storage.

## Key Parameters

- Number of capacitors in the tree (= total caps minus degenerate caps in a loop).
- Number of inductors in the cotree (= total inductors minus those forced into the tree).
- Existence of degenerate situations (capacitor loops, inductor cutsets).

## When To Use

- Setting up state-variable equations for educational or theoretical purposes.
- Network synthesis problems where the number of independent states matters.

## Risks & Pitfalls

- A network with a capacitor-only loop cannot have all capacitors as twigs; this signals a degeneracy where one capacitor voltage is determined by the others.
- Similarly, inductor cutsets indicate that one inductor current is determined by the others.

## Related Concepts

- [[concepts/tree-cotree]]
- [[concepts/state-variable-formulation]]
- [[concepts/cutset-matrix]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
