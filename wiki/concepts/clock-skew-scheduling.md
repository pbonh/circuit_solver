---
title: "Clock Skew Scheduling"
type: concept
tags: [vlsi, digital, synchronization, graph, timing, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/00-preface.txt"]
confidence: medium
---

## Definition

Clock skew scheduling is the process of intentionally assigning non-uniform clock arrival times to clocked elements (flip-flops, latches) within a synchronous digital circuit to improve timing performance, relax setup/hold constraints, or balance critical paths.

## How It Works

The synchronous logic is represented as a timing graph whose nodes are clocked gates and whose edges represent combinational datapaths with associated propagation delays. Clock skew scheduling solves an optimization (often a linear program or shortest/longest path computation on the timing graph) that selects clock arrival times satisfying setup and hold constraints for every datapath while optimizing an objective such as minimum clock period or robustness.

## Key Parameters

- Setup and hold time constraints at each register.
- Combinational path delays (min and max).
- Target clock period.
- Permitted skew range per gate.

## When To Use

- High-performance synchronous designs where uniform (zero-skew) clocking leaves performance on the table.
- As a preprocessing step before clock tree topology generation and physical embedding.

## Risks & Pitfalls

- Skew schedules sensitive to process variation may fail in silicon.
- Hold-time violations can be created if skew is applied carelessly.
- Optimization can be computationally expensive on very large timing graphs.

## Related Concepts

- [[concepts/timing-graph]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/graph-theory]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-00-preface]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
