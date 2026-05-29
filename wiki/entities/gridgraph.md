---
title: GridGraph
type: entity
id: entity-gridgraph
tags:
- graph
- graph-processing
- single-machine
- out-of-core
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

GridGraph (Zhu et al., USENIX ATC 2015) is a single-PC out-of-core graph-processing system that partitions the adjacency matrix as a P×P grid of edge blocks and processes blocks column by column, fusing scatter and gather into a single streaming-apply pass per iteration.

## Characteristics

- Each block (Ia, Ib) holds edges with source in Ia and destination in Ib, stored in arbitrary order; blocks appended into one large file with recorded boundaries to keep sequential bandwidth.
- Column-oriented processing pins the destination vertex chunk Ib in memory across all blocks of a column.
- Disk writes per iteration are O(|V|) (only vertex chunks), not O(|E|) as in GraphChi/X-Stream.
- Supports asynchronous mode by dropping iteration-number bookkeeping for vertex chunks.
- Two-level grid (within each block, a Q×Q sub-grid) improves CPU cache locality.

## Common Strategies

- Use for medium-size graphs where streaming-apply fits the algorithm and write-amplification matters.
- Tune P (block grid resolution) so that two vertex chunks fit in memory simultaneously.
- Enable asynchronous mode for monotone algorithms (Hash-Min).

## Related Entities

- [[entities/graphchi]]
- [[entities/x-stream]]
- [[entities/venus]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
