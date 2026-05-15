---
title: "Interconnect Routing"
type: concept
tags: [vlsi, physical-design, graph, well-established, routing]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

Interconnect routing is the VLSI physical-design step that synthesizes the wires connecting placed circuit components, subject to manufacturing rules, layer constraints, and performance targets. Most modern flows restrict wires to orthogonal (Manhattan) directions.

## How It Works

Modern routing is hierarchical: global routing decomposes the layout into tiles or channels and assigns nets to coarse routes through a channel or switchbox connectivity graph, then detailed routing produces exact wire segments and via placements typically on multiple metal layers. Common algorithms include BFS-based maze routing (Lee, 1961), A*-guided maze routing, channel routing with vertical/horizontal constraint graphs, and rectilinear Steiner tree-based net routing on the Hanan grid.

## Key Parameters

- Number of nets and pin counts.
- Layer count and capacity.
- Wire pitch, design rules.
- Congestion, timing, signal integrity targets.

## When To Use

- Final physical-design stage before manufacturing.
- Iteratively with placement to close timing and congestion.

## Risks & Pitfalls

- Detours due to congestion degrade timing.
- Crosstalk and electromigration constraints may not be captured by pure shortest-path objectives.

## Related Concepts

- [[concepts/maze-routing]]
- [[concepts/a-star-algorithm]]
- [[concepts/steiner-minimal-tree]]
- [[concepts/hanan-grid]]
- [[concepts/placement]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
- [[summaries/graphs-in-vlsi-18-c-multilayer-routing-algorithm]]
