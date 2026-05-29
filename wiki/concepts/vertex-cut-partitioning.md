---
title: Vertex-Cut (Edge) Partitioning
type: claim
id: claim-vertex-cut-partitioning
tags:
- graph
- distributed-systems
- graph-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.85
---

## Definition

Vertex-cut partitioning (a.k.a. edge partitioning) assigns each edge of a graph to exactly one machine, allowing a vertex to be replicated across machines that hold its adjacent edges. The objective is to minimize the total vertex-replication count subject to balance constraints, in contrast to traditional edge-cut partitioning which assigns vertices to machines and counts crossing edges.

## How It Works

Let A(v) be the set of machines containing at least one edge adjacent to v. The total replication N = Σ |A(v)|; the replication factor is N/|V|. PowerGraph's greedy heuristic processes edges (u,v) sequentially:
- If A(u) ∩ A(v) ≠ ∅: place edge on the least-loaded machine in the intersection.
- Else if A(u) ∪ A(v) ≠ ∅: place on least-loaded machine in the union.
- Else: place on the globally least-loaded machine.

Workers run the heuristic in parallel and periodically synchronize A(v). For power-law graphs this dramatically reduces replication compared to hash-based vertex partitioning.

## Key Parameters

- Load-balance tolerance.
- Replication-factor target.
- Whether vertices' A(v) are coordinated synchronously or eventually.

## When To Use

- GAS-style runtimes (PowerGraph and successors) where edges are the unit of work.
- Power-law graphs where a few vertices have very high degree.
- Settings where minimizing memory replicas of vertex state matters more than minimizing message volume.

## Risks & Pitfalls

- Vertex replicas must be kept in sync each iteration, adding network cost.
- Greedy ordering creates dependencies between machines during partitioning.
- Edge partitioning may pessimize cache locality compared to vertex partitioning for some algorithms.

## Related Concepts

- [[concepts/gas-model]]
- [[concepts/graph-partitioning]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
