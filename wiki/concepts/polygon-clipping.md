---
title: "Polygon Clipping"
type: concept
tags: [algorithm, computational-geometry, well-established, routing]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: low
---

## Definition

Polygon clipping is the computational-geometry operation that computes the intersection (or difference, union, XOR) of one polygon with another. Standard algorithms include Sutherland-Hodgman, Weiler-Atherton, Vatti, and Greiner-Hormann; modern implementations are linear in the number of vertices.

## How It Works

Inputs are two polygons defined by ordered vertex lists; outputs are one or more polygons describing the requested Boolean combination. Practical implementations handle holes, self-intersections, and degenerate cases via robust orientation predicates.

## Key Parameters

- Number of vertices.
- Boolean operation type (and, or, not, xor).
- Robustness to degenerate input.

## When To Use

- VLSI/PCB layout Boolean operations (e.g., computing routable available space from blockages).
- GIS spatial analysis.
- Rendering and clipping operations.

## Risks & Pitfalls

- Floating-point round-off creates degenerate intersections; robust geometric kernels are essential.
- Performance degrades with very many short edges.

## Related Concepts

- [[entities/sprout]]
- [[concepts/interconnect-routing]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
