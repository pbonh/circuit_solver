---
title: 'Graphs in VLSI — Appendix C: Multilayer Routing Algorithm'
type: source
id: source-graphs-in-vlsi-18-c-multilayer-routing-algorithm
kind: derived-summary
tags:
- vlsi
- routing
- algorithm
- novel
- board
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/18-c-multilayer-routing-algorithm.txt
---

## Key Points

- When SPROUT's per-layer available space is disjoint, multilayer routing through vias is required. This appendix decomposes the multilayer routing problem into a sequence of single-layer routing problems.
- Construct a 3D graph Γ_n3D by stacking the per-layer 2D graphs Γ_n^l (l = 1, ..., L). Vertical edges connect corresponding vertices across adjacent layers with weight w_via reflecting the higher cost of a via vs. an intra-layer edge.
- Apply a standard shortest-path algorithm (Dijkstra or Bellman-Ford) on Γ_n3D to find the least-cost path between source and target. The path traverses some intra-layer edges and some vertical (via) edges.
- Each via on the resulting path becomes a terminal on the layers it touches. The multilayer problem is now a collection of independent single-layer routing problems (source-to-first-via, between-vias, last-via-to-target) each solved by the regular SPROUT pipeline.
- Higher via cost discourages excessive layer transitions, keeping the number of vias small.

## Relevant Concepts

- [[concepts/multilayer-routing]] — main subject of this appendix.
- [[entities/sprout]] — the tool extended by this method.
- [[concepts/dijkstras-algorithm]] — used to find the multilayer shortest path.
- [[concepts/bellman-ford-algorithm]] — alternative if negative weights arise.
- [[concepts/interconnect-routing]] — broader context.
- [[concepts/board-level-routing]] — typical use case.

## Source Metadata

- Source type: book appendix
- Book title: Graphs in VLSI
- Chapter: Appendix C — Multilayer routing algorithm
- File path: `raw/GraphsInVLSI/_txt/18-c-multilayer-routing-algorithm.txt`
- Authors: Rassul Bairamkulov and Eby G. Friedman
