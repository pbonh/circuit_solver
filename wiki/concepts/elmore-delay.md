---
title: Elmore Delay
type: claim
id: claim-elmore-delay
tags:
- vlsi
- timing
- interconnect
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
confidence:
  base: 0.85
---

## Definition

Elmore delay (W. C. Elmore, 1948) is a first-moment approximation of RC interconnect propagation delay. For a tree, the delay from a source node to a sink node v is t_{u,v} = Σ_{e on u-v path} r_e (c_e/2 + C_v(e)), where r_e is the edge resistance, c_e the edge capacitance, and C_v(e) the downstream capacitance at edge e.

## How It Works

Elmore delay corresponds to the first moment of the impulse response of a linear RC network. It is exact for the centroid of the impulse response and produces an upper-bound-like approximation of step-response delay (typically slightly pessimistic). It enables closed-form analysis of RC trees in O(|tree|) and underpins many CTS, buffer-insertion, and wire-sizing algorithms.

## Key Parameters

- Wire resistance per unit length.
- Wire capacitance per unit length.
- Tree topology and branch capacitances.
- Load capacitances at leaves.

## When To Use

- Quick delay estimation in clock tree synthesis (DME extensions).
- Repeater insertion and wire sizing heuristics.
- Early-stage timing-driven placement.

## Risks & Pitfalls

- Inaccurate for high-frequency signals where higher-order moments matter.
- Does not capture inductance or transmission-line effects.
- Pessimism varies with topology.

## Related Concepts

- [[concepts/deferred-merge-embedding]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/static-timing-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
