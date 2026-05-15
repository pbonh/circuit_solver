---
title: "Infinity Mirror Technique"
type: concept
tags: [vlsi, power-integrity, graph, analysis, novel]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/00-preface.txt"]
confidence: medium
---

## Definition

The Infinity Mirror Technique (IMT) is a constant-time mesh analysis algorithm for IR drop in practical on-chip power networks. It extends infinite lattice graph models — which give closed-form effective resistance expressions — to finite truncated grids by adding image sources that correct boundary inaccuracies.

## How It Works

A regular on-chip power grid is approximated as an infinite lattice graph with closed-form Green's function. Near the boundaries of a finite grid, this approximation degrades significantly (worst-case ~40% error). IMT adds reflected image current sources (analogous to the method of images in electrostatics) that mirror real sources across boundaries. The superposition of the original infinite-grid response with the image responses recovers the correct finite-grid boundary conditions, reducing worst-case error to ~4% while preserving constant-time evaluation per node of interest.

## Key Parameters

- Grid dimensions and regularity.
- Number and placement of image sources.
- Target accuracy versus runtime tradeoff.
- Locations of nodes of interest (only a few need to be evaluated).

## When To Use

- IR drop analysis of large regular on-chip power grids when only a few nodal voltages are needed.
- Inner loops of placement/optimization algorithms (e.g., distributed voltage regulator placement) that require many fast grid evaluations.

## Risks & Pitfalls

- Assumes a sufficiently regular grid topology; highly irregular grids may not benefit.
- Accuracy depends on number of image sources used.
- Not directly suited to transient or coupled inductive analysis without extension.

## Related Concepts

- [[concepts/ir-drop-analysis]]
- [[concepts/power-distribution-network]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-00-preface]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
- [[summaries/graphs-in-vlsi-16-a-green-s-function-for-a-truncated-grid]]
- [[summaries/graphs-in-vlsi-17-b-uniqueness-based-on-boundary-conditions]]
