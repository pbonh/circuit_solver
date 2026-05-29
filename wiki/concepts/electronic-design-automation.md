---
title: Electronic Design Automation (EDA)
type: claim
id: claim-electronic-design-automation
tags:
- vlsi
- eda
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/04-1-introduction.txt
confidence:
  base: 0.85
---

## Definition

Electronic Design Automation (EDA) is the collection of software tools and methodologies used to design, verify, and validate integrated circuits and electronic systems. EDA grew out of CAD in the late 1960s and 1970s as IC complexity outpaced manual design.

## How It Works

EDA tools span the full IC design hierarchy: behavioral synthesis, logic synthesis, technology mapping, placement, routing, timing analysis, power analysis, and physical verification. Many EDA algorithms reduce to graph problems (partitioning, coloring, shortest paths, spanning trees, max flow) operating on netlists, timing graphs, or layout graphs. The 1970s saw commercial EDA emerge through companies like Daisy Systems, Mentor Graphics, and Valid Logic Systems, growing into a multi-billion-dollar industry (~$9B by 2020).

## Key Parameters

- Design size (gate count, transistor count).
- Tool runtime and memory usage.
- Quality-of-results metrics: area, power, performance, yield.
- Iteration count required for closure.

## When To Use

- Any non-trivial IC design where manual layout and verification are infeasible.
- Required throughout the modern VLSI design flow from RTL to GDS-II.

## Risks & Pitfalls

- Tool flow complexity and licensing costs.
- Quality of results depends heavily on heuristic settings and designer expertise.
- Inter-tool data exchange (formats, abstractions) is a frequent source of error.

## Related Concepts

- [[concepts/vlsi-design]]
- [[concepts/graph-theory]]
- [[entities/spice]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/integrated-circuit]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
