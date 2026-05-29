---
title: Proxy Graph
type: claim
id: concepts/proxy-graph
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

In QuCTS, a proxy graph G_p = (V_p, E_p, w) abstracts the layout regions available for splitter and JTL placement between two clock sinks A and B. Nodes are A, B, and candidate gate-cell locations within a corridor along the line connecting them. Edges connect all node pairs except {A, B}, with edge weight equal to the Manhattan distance between the corresponding layout positions.

## How It Works

By restricting edges to candidate cell locations, the proxy graph captures the discrete placement choices for the splitter and intermediate delay elements. The k-shortest-paths algorithm enumerates candidate proxy paths in increasing edge-weight order; each is analyzed for delay-equilibration mismatch ε(g_k). The proxy path with the smallest mismatch (within tolerance) and fewest delay elements becomes the basis for fine routing on the Hanan grid.

## Key Parameters

- Corridor width around the A-B line.
- Number of candidate cells included.
- Exploration parameter k controlling number of delay-element candidates.

## When To Use

- Splitter and delay-element placement in RSFQ clock-tree synthesis.
- General routing problems where placement candidates lie on a sparse layout grid.

## Risks & Pitfalls

- Coarse corridor selection may miss good candidate locations.
- Number of proxy paths grows quickly with cell count.

## Related Concepts

- [[entities/qucts]]
- [[concepts/k-shortest-path-algorithm]]
- [[concepts/hanan-grid]]
- [[concepts/clock-tree-synthesis]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
