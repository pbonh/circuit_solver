---
title: "NScale"
type: entity
tags: [graph, distributed-systems, graph-mining, mapreduce, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: medium
---

## Overview

Per Systems For Big Graph Analytics Chapter 6 ("Subgraph-Centric Graph Mining"): "NScale [9] only supports the top-level decomposed subgraphs, and there is no mechanism to balance workload through recursive decomposition. Assuming that each decomposed subgraph G_i spans the k-hop neighborhood around each vertex v_i, then NScale first constructs all decomposed subgraphs using k rounds of MapReduce. The large number of decomposed subgraphs are then packed into larger compact subgraphs, each of which can fit in the memory of a reducer. Vertices common to multiple decomposed subgraphs are stored only once in their packed subgraph. Finally, each compact subgraph is distributed to a reducer, which processes all decomposed subgraphs packed in the compact subgraph in memory." The book criticizes the design: "NScale suffers from all the performance issues of a vertex-centric solution, as well as the huge overhead of repeated HDFS data loading/dumping." Reference: A. Quamar, A. Deshpande, J. Lin, "NScale: neighborhood-centric large-scale graph analytics in the cloud," VLDB Journal 25(2):125–150, 2016.

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
- [[concepts/mapreduce]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
