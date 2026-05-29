---
title: Anisotropic Resistive Grid
type: claim
id: claim-anisotropic-grid
tags:
- vlsi
- power-integrity
- graph
- analysis
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/09-6-effective-resistance-of-truncated-infinite-mesh-structures.txt
confidence:
  base: 0.65
---

## Definition

An anisotropic resistive grid is a 2D (or 3D) resistive lattice whose horizontal and vertical (and depth) edge resistances differ. The anisotropy ratio is conventionally k = r_y / r_x where r_x is the horizontal edge resistance and r_y is the vertical edge resistance.

## How It Works

VLSI power distribution networks frequently have different metal pitches and widths in the horizontal and vertical directions, producing different per-edge resistance. The Green's function for an anisotropic infinite lattice generalizes the isotropic form: R(x, y, k) = (kr/π) ∫_0^π (1 − e^{−|x|α} cos(yβ)) / sinh(α) dβ with k + 1 = k cos(β) + cosh(α). Closed-form approximations express the dependence on k as a low-order polynomial.

## Key Parameters

- Horizontal and vertical resistance per segment.
- Anisotropy ratio k.
- Grid dimensions.

## When To Use

- IR drop analysis of layered power grids with different per-direction wire pitches.
- Substrate noise modeling with directionally-varying conductivity.
- Any uniform lattice with different intra-axis edge weights.

## Risks & Pitfalls

- Strong anisotropy (k far from 1) widens polynomial-approximation error; coefficient tables must be valid for the target k range.
- 3D anisotropy further complicates closed-form expressions.

## Related Concepts

- [[concepts/lattice-graph]]
- [[concepts/lattice-greens-function]]
- [[concepts/infinity-mirror-technique]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
