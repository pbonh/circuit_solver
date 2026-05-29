---
title: PEGASUS
type: entity
id: entity-pegasus
tags:
- graph
- big-data
- sparse-matrix
- mapreduce
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
---

## Overview

PEGASUS (Kang, Tsourakakis, Faloutsos, ICDM 2009; KAIS 2011) is a peta-scale graph-mining system developed at CMU, one of the very first general-purpose big graph systems and a pre-Pregel pioneer. It models each iteration of graph computation as a generalized matrix-vector multiplication implemented over MapReduce. Open source at `cs.cmu.edu/~pegasus`, though no commits since 2010.

## Characteristics

- MapReduce-based runtime: each iteration is two MapReduce jobs (combine2 emission then group-by-row reduction).
- Generalized matrix-vector multiplication parameterized by user UDFs `combine2`, `combineAll`, `assign`.
- Partitions adjacency matrix into b×b square submatrices and the vector into b-element blocks.
- Co-clustering of rows and columns to compact non-zeros into fewer submatrices.
- Repeated diagonal-block multiplication propagates state inside the block, reducing iteration count for monotone algorithms like Hash-Min.

## Common Strategies

- Express algorithms using sum/min/etc. as combineAll for PageRank, Hash-Min, Connected Components.
- Apply co-clustering preprocessing once and reuse across queries.
- Switch to a vertex-centric or SystemML for non-MR workloads, since MR job startup overhead dominates small graphs.

## Related Entities

- [[concepts/mapreduce]]
- [[entities/gbase]]
- [[entities/systemml]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
