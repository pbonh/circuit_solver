---
title: "Tree and Cotree"
type: concept
tags: [foundational, graph, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt"]
confidence: high
---

## Definition

A tree of a connected oriented graph with n+1 nodes is a connected subgraph that contains all nodes and no closed paths; it has exactly n edges, called twigs. The remaining b - n edges form the cotree; they are called chords.

## How It Works

The tree induces a fundamental decomposition:
- One basic cut per twig (intersects exactly one twig and arbitrary chords).
- One basic loop per chord (closed by inserting the chord into the tree).

The chord currents (b - n of them) form the independent current variables; the twig voltages (n of them) form the independent voltage variables. This is the basis of cutset, loopset, and state-variable formulations.

## Key Parameters

- Tree size n (number of twigs).
- Cotree size b - n (number of chords).
- Choice of tree (not unique; a network has many trees, often counted by Kirchhoff's matrix-tree theorem).
- Recommended numbering: twigs first (1..n), then chords (n+1..b).

## When To Use

- Any topological formulation of network equations.
- State-variable formulation: select a normal tree containing voltage sources and as many capacitors as possible.
- Symbolic analysis (Chapter 7 in the book) where tree enumeration arises.

## Risks & Pitfalls

- Some networks restrict which elements can be twigs (e.g., a network containing a capacitor loop has no tree with all capacitors as twigs).
- Cutset/loopset matrices can be dense even when the nodal admittance matrix is sparse.

## Related Concepts

- [[concepts/oriented-graph]]
- [[concepts/cutset-matrix]]
- [[concepts/loopset-matrix]]
- [[concepts/state-variable-formulation]]
- [[concepts/normal-tree]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
