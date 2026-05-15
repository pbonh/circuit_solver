---
title: "Static Timing Analysis (STA)"
type: concept
tags: [vlsi, digital, timing, graph, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: high
---

## Definition

Static timing analysis (STA) is a graph-based method for verifying the timing of a digital circuit by computing worst-case path delays without performing electrical simulation. It identifies whether every datapath meets setup and hold constraints across operating corners.

## How It Works

STA traverses the timing graph (registers as nodes, combinational datapaths as edges) computing forward arrival times and backward required times for each node. Slack = required − arrival; negative slack indicates a timing violation. PERT (developed by the US Navy in 1958 for project scheduling, adapted to ICs in 1965) is regarded as the first STA tool. Modern STA tools handle multi-corner, multi-mode operation, on-chip variation, and crosstalk-induced delay.

## Key Parameters

- Min/max delay per cell and per net.
- Setup, hold, recovery, removal constraints.
- Clock arrival time and uncertainty.
- Operating corners (PVT).

## When To Use

- Sign-off timing verification for every digital IC.
- Iteratively during physical design to close timing.

## Risks & Pitfalls

- Pessimism in delay margins inflates iteration count.
- False paths and multi-cycle paths require explicit specification.
- Dynamic effects (IR drop, crosstalk) not fully captured by pure STA.

## Related Concepts

- [[concepts/timing-graph]]
- [[concepts/clock-skew-scheduling]]
- [[concepts/constraint-graph]]
- [[concepts/elmore-delay]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
