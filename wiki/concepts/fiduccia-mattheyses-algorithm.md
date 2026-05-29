---
title: Fiduccia-Mattheyses Algorithm
type: claim
id: concepts/fiduccia-mattheyses-algorithm
tags:
- graph
- algorithm
- partitioning
- vlsi
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Fiduccia-Mattheyses (FM) algorithm (1982) is a graph and hypergraph partitioning heuristic that generalizes Kernighan-Lin by moving one vertex at a time instead of swapping pairs, supporting hypergraphs and balance-constrained (unequal) partitions.

## How It Works

Vertices have a gain — the reduction in cut size if moved across the partition boundary. A bucket data structure indexed by gain allows O(1) gain-max selection. Each pass selects the highest-gain free vertex whose move respects the balance constraints, moves it, locks it, and updates gains of its neighbors. After all vertices are locked, the best prefix of moves is retained. FM passes typically run in O(|pins|) for hypergraphs.

## Key Parameters

- Partition balance constraint (min/max size).
- Number of passes.
- Bucket-list data structure for O(1) maximum-gain selection.

## When To Use

- Standard inner kernel of multilevel partitioners (hMETIS, KaHyPar).
- Hypergraph cut minimization in VLSI netlist partitioning.

## Risks & Pitfalls

- Local optima sensitivity to initialization.
- Quality degrades on highly unbalanced or degenerate inputs without diversification.

## Related Concepts

- [[concepts/graph-partitioning]]
- [[concepts/kernighan-lin-algorithm]]
- [[concepts/hypergraph]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
