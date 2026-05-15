---
title: "NScale"
type: entity
tags: [graph, distributed-systems, graph-mining, mapreduce, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: low
---

## Overview

NScale (Quamar, Deshpande, Lin, VLDB Journal 2016) is a neighborhood-centric large-scale graph-analytics system. It supports only top-level decomposed subgraphs (no recursive decomposition for load balancing) and constructs the per-seed k-hop neighborhoods via k rounds of MapReduce.

## Characteristics

- MapReduce-based; runs on Hadoop.
- Constructs all decomposed subgraphs synchronously via k MapReduce passes for k-hop neighborhoods.
- Packs many small decomposed subgraphs into larger "compact" subgraphs that fit in a reducer's memory, deduplicating shared vertices.
- Each reducer processes all decomposed subgraphs packed in its compact subgraph in-memory.
- No mechanism to balance workload through recursive decomposition.

## Common Strategies

- Use only for problems where k-hop neighborhoods are uniformly small.
- Plan for heavy HDFS read/write overhead from repeated MapReduce passes.
- Prefer G-thinker for computation-intensive workloads on power-law graphs.

## Related Entities

- [[entities/arabesque]]
- [[entities/g-thinker]]
- [[entities/mapreduce]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
