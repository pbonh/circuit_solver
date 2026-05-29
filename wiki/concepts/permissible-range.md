---
title: Permissible Range (Clock Skew)
type: claim
id: claim-permissible-range
tags:
- vlsi
- timing
- synchronization
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
confidence:
  base: 0.85
---

## Definition

The permissible range (PR) of a sequentially-adjacent datapath (i, f) is the interval [l_if, u_if] of clock skew values that guarantees correct synchronous operation: lower bound l_if = -d_if + δ_h^f (hold time / double-clocking limit) and upper bound u_if = T_CP - D_if - δ_s^f (setup time / zero-clocking limit), where d_if and D_if are min/max combinational propagation delays and δ_h^f, δ_s^f are register hold and setup times.

## How It Works

Within the PR, both setup and hold constraints are satisfied. Skew below l_if causes a race condition (double clocking); skew above u_if causes a clock period violation (zero clocking). The PR width is T_CP - δ_s^f - δ_h^f - DS_if where DS_if = D_if - d_if (data skew). The minimum feasible clock period for a single path is T_CP^min = δ_s^f + δ_h^f + DS_if.

## Key Parameters

- Hold and setup times of the destination register.
- Min/max propagation delays of the combinational path.
- Target clock period.

## When To Use

- Inner constraint in clock skew scheduling.
- Per-path verification during static timing analysis.

## Risks & Pitfalls

- Process variation perturbs both bounds; statistical PRs require Gaussian or arbitrary-distribution modeling.
- Tight PRs leave little robustness margin against environmental variation.

## Related Concepts

- [[concepts/clock-skew-scheduling]]
- [[concepts/timing-graph]]
- [[concepts/constraint-graph]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
