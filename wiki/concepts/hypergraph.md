---
title: Hypergraph
type: claim
id: concepts/hypergraph
tags:
- graph
- foundational
- well-established
- vlsi
- netlist
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A hypergraph H = (V, E, ψ) generalizes a graph by allowing each hyperedge to connect an arbitrary nonzero subset of nodes rather than a pair: ψ : E → P(V) \ {∅}. Every graph is a hypergraph, but not vice versa.

## How It Works

Hyperedges encode "multi-adic" relationships such as a wire that fans out to many gate inputs in a VLSI netlist. In partitioning problems, hypergraph models are more accurate than reducing each net to a clique of pairwise edges. Specialized algorithms (hMETIS, KaHyPar) directly operate on hypergraphs.

## Key Parameters

- Number of nodes |V|, number of hyperedges |E|.
- Hyperedge size distribution (average and max pins per net).
- Vertex and hyperedge weights.

## When To Use

- Netlist modeling for partitioning, floorplanning, placement.
- Multi-party relationships (cellular interference, biological pathways, computational social science).
- When pairwise reduction would distort the underlying problem.

## Risks & Pitfalls

- Direct hypergraph algorithms are more complex than graph algorithms.
- Many implementations reduce to a graph (clique or star expansion) at quality cost.
- Hyperedge cut metrics differ (cut, sum-of-external-degrees, K-1) and choice impacts results.

## Related Concepts

- [[concepts/graph-theory]]
- [[concepts/graph-partitioning]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/guide-to-graph-algorithms-05-algorithms]]
