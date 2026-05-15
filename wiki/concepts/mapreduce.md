---
title: "MapReduce"
type: concept
tags: [batch, distributed-systems, well-established, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: medium
---

## Definition

MapReduce is a programming model for processing large datasets in bulk across many machines, popularized by Google. The user supplies two pure functions: `map`, called once per input record to emit key-value pairs, and `reduce`, called once per distinct key to aggregate the emitted values. It is neither fully declarative nor fully imperative, sitting between the two.

## How It Works

- `map(record) -> [(key, value), ...]` is applied per input record. Outputs are grouped by key by the framework.
- `reduce(key, [values]) -> result` is called once per group, producing aggregated output.
- Functions must be pure (no side effects, no external queries) so the framework can rerun them anywhere and on failure.
- MongoDB and CouchDB exposed MapReduce as a read-only query mechanism over document collections, but MongoDB later added a declarative aggregation pipeline that is easier to optimize.
- Higher-level languages (SQL, Hive, Pig) can be compiled into pipelines of MapReduce stages; Chapter 10 of DDIA covers this in depth.

## Key Parameters

- Number of map/reduce workers and partitioning function.
- Materialization of intermediate state (disk vs memory).
- Combiner functions for partial aggregation before the reduce phase.

## When To Use

For batch analytics over large immutable datasets when the operations decompose cleanly into map and reduce phases. Less attractive today than dataflow engines like Spark or Flink for interactive workloads, but the model remains the conceptual foundation.

## Risks & Pitfalls

- Two coordinated user functions are harder to write than a single declarative query.
- Without a query optimizer, performance tuning is manual.
- Intermediate materialization makes multi-stage pipelines slow vs in-memory dataflow engines.

## Related Concepts

- [[concepts/declarative-query-language]]
- [[concepts/data-warehouse]]
- [[concepts/column-oriented-storage]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-05-part-iii-derived-data]]
