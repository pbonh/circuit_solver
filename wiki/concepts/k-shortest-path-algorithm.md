---
title: "k-Shortest Paths Algorithm"
type: concept
tags: [graph, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt"]
confidence: medium
---

## Definition

The k-shortest paths problem extends single-pair shortest path to enumerate the k loopless paths from source to target in increasing order of total weight. Classical algorithms include Yen's algorithm (1971) and Eppstein's algorithm (1998).

## How It Works

Yen's algorithm uses k-1 invocations of Dijkstra plus path-deviation heuristics; runtime is O(k·|V|(|E| + |V| log |V|)). Eppstein's algorithm achieves O(|E| + |V| log |V| + k) for the more general (with-repeats) variant. The variants used in QuCTS specifically enumerate simple (loopless) paths from A to B in the proxy graph.

## Key Parameters

- Number of paths k requested.
- Whether paths must be simple (no repeated vertices).
- Underlying shortest-path algorithm (Dijkstra, Bellman-Ford).

## When To Use

- Generating candidate routes for evaluation (e.g., QuCTS proxy paths).
- Alternative-route generation in navigation systems.
- Failure-tolerant network design.

## Risks & Pitfalls

- Large k makes both runtime and memory grow rapidly.
- Yen's algorithm performance degrades for very dense graphs.

## Related Concepts

- [[concepts/dijkstras-algorithm]]
- [[concepts/proxy-graph]]
- [[entities/qucts]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
