---
title: Lattice Green's Function
type: claim
id: concepts/lattice-greens-function
tags:
- graph
- vlsi
- analysis
- well-established
- mathematics
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A lattice Green's function gives a closed-form solution for the response (potential, effective resistance, propagator) of a regular lattice to a unit point source. For an infinite uniform 2D resistive lattice with unit resistance r, the effective resistance between two nodes separated by (x, y) is given by integral expressions due to McCrea & Whipple (1940), van der Pol (1933), Spitzer (1976), and Cserti (2000).

## How It Works

Closed-form expressions include R(x, y) = (1/π) ∫_0^π (1 − e^{-xμ} cos(yλ)) / sinh(μ) dλ with cosh(μ) + cos(λ) = 2 (uniform 2D grid). Anisotropic generalizations introduce a ratio k of horizontal to vertical resistance (Eq. 5.73 in the book): R(x, y, k) = (kr/π) ∫_0^π (2 − e^{-|x|α} cos(yβ)) / sinh(α) dβ with k + 1 = k cos(β) + cosh(α). For adjacent nodes, R = r/2 by symmetry and superposition.

## Key Parameters

- Source-target separation (x, y).
- Anisotropy ratio k.
- Resistance per element r.
- Lattice dimension (2D, 3D).

## When To Use

- Analytical IR drop estimation in large regular power grids.
- Initialization or correction term in numerical solvers.
- Foundation for the Infinity Mirror Technique applied to finite grids.

## Risks & Pitfalls

- Assumes infinite grid; near-boundary error in finite grids is significant (~40% worst case without correction).
- Closed-form integrals require careful numerical evaluation.

## Related Concepts

- [[concepts/lattice-graph]]
- [[concepts/effective-resistance]]
- [[concepts/infinity-mirror-technique]]
- [[concepts/ir-drop-analysis]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-16-a-green-s-function-for-a-truncated-grid]]
