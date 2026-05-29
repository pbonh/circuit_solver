---
title: Method of Images
type: claim
id: claim-method-of-images
tags:
- analysis
- foundational
- well-established
- mathematics
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/09-6-effective-resistance-of-truncated-infinite-mesh-structures.txt
confidence:
  base: 0.65
---

## Definition

The method of images is a mathematical technique for solving boundary-value problems by replacing the boundary with one or more fictitious "image" sources placed in mirror positions across the boundary. The combined field of real and image sources automatically satisfies the boundary condition, reducing the boundary-value problem to a free-space superposition.

## How It Works

Classical electrostatic applications place an image charge of opposite sign across a grounded conducting plane so that the equipotential surface coincides with the conductor. In *Graphs in VLSI*, the method is adapted to resistive lattices: image current sources are placed across a mesh truncation to satisfy φ(0, y) − φ(−1, y) = 0 (no current crosses the boundary). The total potential is the superposition of free-space lattice Green's functions evaluated at the real and image source positions.

## Key Parameters

- Number and placement of image sources.
- Boundary geometry (planar, multi-planar, curved).
- Underlying free-space Green's function.

## When To Use

- Boundary-corrected effective-resistance computation in truncated resistive lattices (Infinity Mirror Technique).
- Electrostatics with planar/spherical conductors.
- Acoustic and electromagnetic scattering with regular boundaries.

## Risks & Pitfalls

- Only applicable when the underlying free-space problem and the boundary symmetry admit closed-form image placement.
- Multiple-boundary problems may require infinitely many images (e.g., parallel-plate corners).

## Related Concepts

- [[concepts/infinity-mirror-technique]]
- [[concepts/lattice-graph]]
- [[concepts/lattice-greens-function]]
- [[concepts/effective-resistance]]

## Sources

- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-16-a-green-s-function-for-a-truncated-grid]]
- [[summaries/graphs-in-vlsi-17-b-uniqueness-based-on-boundary-conditions]]
