---
title: Aura Snaking
type: claim
id: concepts/aura-snaking
tags:
- vlsi
- routing
- algorithm
- novel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Aura snaking is a wire-snaking technique introduced in QuCTS for adding controlled propagation delay to a clock path. The "aura" is the set of points within a distance d of the wire segment; congestion-aware selection of aura points produces non-self-intersecting detours that lengthen the wire by 2d per iteration.

## How It Works

At each iteration: (1) generate aura points within distance d of the current wire segment, excluding any inside blockages; (2) compute a proximity metric p_q = (Σ 1/||pq||_s)^{-1} discouraging selection of points near other cells (congestion-aware); (3) replace the wire segment adjacent to the chosen aura points with a detour through those points. Delay increase per iteration is 2d/v where v is the SFQ propagation speed. The final iteration uses d* = v|t_A - t_B|/2 for exact arrival-time matching.

## Key Parameters

- Aura distance d (per iteration).
- Proximity-metric norm s.
- Maximum number of iterations.

## When To Use

- Fine adjustment of clock-arrival delays in RSFQ clock tree routing.
- Any wire-length-controlled timing tuning where exact delay must be achieved.

## Risks & Pitfalls

- Congested layouts limit aura-point options.
- Excessive snaking increases parasitic capacitance.

## Related Concepts

- [[entities/qucts]]
- [[concepts/hanan-grid]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/proxy-graph]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
