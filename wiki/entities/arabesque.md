---
title: Arabesque
type: entity
id: entities/arabesque
tags:
- graph
- distributed-systems
- graph-mining
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
---

## Overview

Arabesque (Teixeira et al., SOSP 2015) is a distributed graph-mining system organized around the concept of "embeddings" (subgraphs of the input graph). It grows embeddings synchronously from small to large, filtering at each step, and materializes every candidate embedding examined.

## Characteristics

- Embedding-centric programming model.
- Requires the entire input graph to fit in every machine's RAM, limiting scalability to a single machine's memory.
- Iteratively grows embeddings by one edge/vertex per iteration; filters via user-provided predicate (e.g., "is a clique").
- Compresses materialized embeddings using ODAG data structure to save space.
- Performs automorphism checking for every new embedding to avoid duplicates (overhead for clique/quasi-clique problems that do not need it).

## Common Strategies

- Use for frequent-subgraph mining where automorphism deduplication is needed.
- Plan for memory consumption proportional to the number of candidate embeddings.
- Avoid for problems addressable by subgraph-centric backtracking (G-thinker is reported much faster and more scalable).

## Related Entities

- [[entities/g-thinker]]
- [[entities/nscale]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
