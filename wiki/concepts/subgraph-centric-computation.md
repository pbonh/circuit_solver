---
title: "Subgraph-Centric Computation"
type: concept
tags: [graph, distributed-systems, graph-mining, big-data, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: high
---

## Definition

Subgraph-centric computation is a parallel graph-mining model in which the unit of work is a subgraph (typically a per-seed decomposed subgraph such as a k-hop neighborhood or 1-ego network), and each task runs a serial backtracking algorithm on its subgraph to find target patterns (cliques, quasi-cliques, dense subgraphs, motifs, graph matches).

## How It Works

For a problem like maximal-clique enumeration, the algorithm creates one task per seed vertex v_i, whose subgraph G_i is induced by {v_i} ∪ Γ_>(v_i) (neighbors with larger IDs, eliminating double-counting). Each task pulls the required vertices and adjacency lists from local or remote workers and then runs a sequential backtracking enumerator on G_i. When a G_i is too large (power-law tails), it is further decomposed recursively. The framework (e.g., G-thinker) schedules tasks asynchronously across workers with bounded memory using a disk-based task queue, an in-memory batch of active tasks, and an LRU cache of remotely fetched vertices shared by all tasks on a worker.

## Key Parameters

- Decomposition rule (1-ego, k-ego, conditional on prefix vertex).
- Task batch size (memory vs. throughput tradeoff).
- Frequency of optional aggregator synchronization (e.g., current best clique size).
- Whether to support recursive subgraph decomposition for load balancing.
- Communication-pruning UDF `respond(v)` for adjacency-list trimming.

## When To Use

- Computation-intensive graph-mining problems (maximum clique, maximal clique enumeration, quasi-clique, motif counting, subgraph matching).
- Workloads where vertex-centric or block-centric models would be data-intensive or require materializing every candidate embedding.
- Problems where the input graph is too large for every machine to hold in memory (unlike Arabesque).

## Risks & Pitfalls

- Power-law degree distributions create occasional huge G_i that become stragglers; recursive decomposition is essential.
- Tasks can overlap in memory footprint; without LRU caching the same vertex is fetched many times.
- Without aggregator-based pruning, branch-and-bound algorithms (e.g., max-clique) explore a much larger search space.
- Implementation complexity exceeds Pregel-like systems.

## Related Concepts

- [[concepts/maximal-clique-enumeration]]
- [[concepts/computation-intensive-vs-data-intensive]]
- [[concepts/task-batching]]
- [[concepts/lru-vertex-cache]]
- [[concepts/vertex-centric-programming]]

## Sources

- [[summaries/systems-big-graph-analytics-01-1-introduction]]
- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
