---
title: Lattice Graph
type: claim
id: claim-lattice-graph
tags:
- graph
- vlsi
- foundational
- well-established
- power-integrity
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
---

## Definition

A lattice graph is a graph whose vertices are arranged at the points of a regular lattice (typically Z^d) and whose edges connect each vertex to its lattice-neighbors. In VLSI, two-dimensional (and three-dimensional) lattice graphs model regular power and clock distribution networks.

## How It Works

A grid of width x and height y has x+y choose x distinct paths between opposite corners — superlinear redundancy that provides reliability. Lattice graphs admit closed-form effective resistance via lattice Green's functions (McCrea-Whipple 1940; van der Pol 1933). The number of paths grows superlinearly with grid dimensions. Infinite lattice approximations enable constant-time evaluation of effective resistance between arbitrary node pairs sufficiently far from boundaries.

## Key Parameters

- Grid dimensions x, y (or x, y, z).
- Edge weights (uniform or anisotropic).
- Boundary conditions (open, periodic).

## When To Use

- Modeling on-chip power distribution networks.
- Cellular automata and statistical physics.
- Reliability analysis of redundant interconnect.
- As the substrate for the Infinity Mirror Technique.

## Risks & Pitfalls

- Closed-form expressions assume uniformity; practical anisotropy requires extensions (Eq. 5.73 in the book).
- Finite-grid edge effects degrade infinite-grid approximations near boundaries.

## Related Concepts

- [[concepts/lattice-greens-function]]
- [[concepts/effective-resistance]]
- [[concepts/infinity-mirror-technique]]
- [[concepts/power-distribution-network]]
- [[concepts/random-walk]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-17-b-uniqueness-based-on-boundary-conditions]]
