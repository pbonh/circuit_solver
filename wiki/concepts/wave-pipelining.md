---
title: "Wave Pipelining"
type: concept
tags: [vlsi, digital, timing, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: medium
---

## Definition

Wave pipelining is a synchronous design technique in which the clock period T_CP is less than the propagation delay of a combinational datapath, so multiple data items ("waves") propagate simultaneously through the same logic. Data are spatially rather than temporally separated.

## How It Works

In conventional pipelining, only one datum is in a combinational datapath at a time. Wave pipelining releases the next datum before the previous datum has reached the destination register; correct operation requires the data-skew DS = D_max - D_min to be small enough that waves never overtake one another. The minimum achievable T_CP is bounded below by data skew plus setup and hold times.

## Key Parameters

- Clock period T_CP.
- Data skew DS = D_max - D_min.
- Register setup and hold times.
- Number of simultaneous waves.

## When To Use

- High-throughput datapaths where additional pipeline registers are expensive (area, latency).
- Specialized hardware like floating-point ALUs and DSP datapaths.

## Risks & Pitfalls

- Sensitivity to delay variation: small DS budgets can be violated by PVT effects.
- Verification is more complex than for conventional pipelines.

## Related Concepts

- [[concepts/clock-skew-scheduling]]
- [[concepts/permissible-range]]
- [[concepts/timing-graph]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
