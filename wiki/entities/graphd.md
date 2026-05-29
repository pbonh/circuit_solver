---
title: GraphD
type: entity
id: entity-graphd
tags:
- graph
- distributed-systems
- graph-processing
- out-of-core
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

GraphD (Yan et al., 2016) is the out-of-core Pregel-like system in the BigGraph@CUHK toolkit. It targets commodity PC clusters connected by Gigabit Ethernet, where sequential disk bandwidth far exceeds network bandwidth, and hides disk-streaming time inside message transmission to achieve performance comparable to in-memory Pregel-like systems on much larger graphs.

## Characteristics

- Edges streamed per superstep; sparse-skip mechanism skips k edges when intervening vertices are inactive (degree-sum bookkeeping).
- Outgoing message stream organized as size-bounded file series so head can be read while tail is appended.
- Message-sending thread combines messages on receiver-side as they arrive; receiving thread aggregates partials in memory.
- O(|V|) memory per machine; one pass over disk streams per superstep for combiner-applicable algorithms.
- Distributed via the `ioser.h` header with `ofbinstream` / `ifbinstream` types for disk streaming.

## Common Strategies

- Use when the cluster has limited memory and Gigabit Ethernet (not 40GbE/SSD).
- Pair with Pregel+ application code; the API is similar with disk-streaming-aware extensions.
- Prefer over Chaos when network is slow relative to disk.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/pregel-plus]]
- [[entities/chaos]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
