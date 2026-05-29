---
title: Computation-Intensive vs. Data-Intensive Workloads
type: claim
id: claim-computation-intensive-vs-data-intensive
tags:
- distributed-systems
- big-data
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
confidence:
  base: 0.85
---

## Definition

A workload is data-intensive when communication cost (moving bytes) dominates CPU cost (processing bytes), and computation-intensive when CPU cost per unit of data is large enough that overlapping computation with communication yields high throughput. The distinction guides system-design choices for big graph analytics.

## How It Works

Vertex-centric graph frameworks (Pregel, GraphLab, PowerGraph), MapReduce, and Spark all assume data-intensive workloads: per-record (per-vertex, per-edge, per-row) work is cheap, so the system optimizes for streaming throughput, message combining, and bulk synchronization. Computation-intensive workloads — subgraph finding, maximum clique, graph matching — perform super-linear work on each decomposed subgraph; the system can profitably overlap subgraph computation with on-demand vertex pulling, schedule tasks independently without global barriers, and pipeline I/O behind compute. G-thinker is explicitly designed for this regime; NScale and Arabesque retain a data-intensive flavor and pay a corresponding cost.

## Key Parameters

- Per-unit work complexity (linear in vertex degree vs. exponential in subgraph size).
- Network/disk bandwidth vs. CPU throughput on the target cluster.
- Memory pressure from intermediate state (messages, candidate embeddings).

## When To Use

- As an analysis lens when choosing or designing a graph framework.
- When justifying subgraph-centric (computation-intensive) over vertex-centric (data-intensive) for graph mining.

## Risks & Pitfalls

- Many "graph algorithm" implementations in vertex-centric frameworks become data-intensive due to message buffering even if the underlying problem is computationally hard.
- Misclassification leads to wrong system choices and orders-of-magnitude performance gaps.

## Related Concepts

- [[concepts/subgraph-centric-computation]]
- [[concepts/vertex-centric-programming]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
