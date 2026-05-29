---
title: MapReduce
type: claim
id: claim-mapreduce
tags:
- batch
- distributed-systems
- big-data
- batch-processing
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
confidence:
  base: 0.85
---

## Definition

MapReduce (Dean and Ghemawat, OSDI 2004) is Google's foundational distributed batch-processing model for large datasets. A job is expressed as two pure user-supplied functions — `map(key, value) → [(key', value'), …]` applied once per input record, followed by `reduce(key', [value']) → output` called once per distinct intermediate key — joined by a framework-managed *shuffle* that sorts and groups intermediate `(key', value')` pairs across the cluster. The model sits between fully declarative and fully imperative: pure functions, framework-managed parallelism, but no query optimizer. Apache Hadoop is the canonical open-source implementation, and graph systems such as [[entities/pegasus]], [[entities/gbase]], and [[entities/systemml]] (in MR-backend mode) execute graph operations as sequences of MR jobs.

## How It Works

- **Map.** `map(record) → [(key, value), …]` is applied per input record. Outputs are partitioned and shuffled by key.
- **Shuffle.** The framework sorts and groups intermediate pairs by key across the cluster — the implicit synchronization barrier between phases.
- **Reduce.** `reduce(key, [values]) → result` is called once per group, producing the aggregated output.
- **Purity requirement.** Both functions must be pure (no side effects, no external state, no remote queries) so the framework can re-run them anywhere on failure. This is the foundation of MapReduce's strong fault tolerance: failed tasks are simply re-executed.
- **Materialization.** Intermediate output between successive MR jobs is written to a distributed file system ([[entities/hdfs|HDFS]]). This is what makes multi-stage pipelines slow versus in-memory [[entities/apache-spark|Spark]] / [[entities/apache-flink|Flink]] dataflow engines.
- **Surface compilation.** Higher-level languages (SQL, Hive, Pig) compile down to pipelines of MR stages; DDIA chapter 10 covers this in depth.
- **Document-DB variants.** MongoDB and CouchDB once exposed MapReduce as a read-only query mechanism over document collections; MongoDB later added a declarative aggregation pipeline that is easier to optimize.

## Key Parameters

- Number of map and reduce workers and the partitioning function on intermediate keys.
- Materialization of intermediate state (disk vs memory) — typically disk in classic Hadoop.
- **Combiner** functions for partial aggregation on the map side before the shuffle, reducing intermediate traffic.
- Object reuse pattern reduces per-record allocation cost on the JVM.

## When To Use

- One-shot ETL and batch analytics over large immutable datasets where job latency is dwarfed by data size.
- When operations decompose cleanly into map and reduce phases — counting, grouping, joining via secondary-sort, simple aggregation.
- As an execution substrate for higher-level batch query languages (Hive, Pig, SystemML's DML).
- **Less attractive today** than dataflow engines like Spark or Flink for interactive or iterative workloads, but the model remains the conceptual foundation for those systems.

## Risks & Pitfalls

- **Two coordinated user functions** are harder to write than a single declarative query; performance tuning is largely manual without a query optimizer.
- **Materialization between stages** makes multi-stage pipelines slow versus in-memory dataflow engines; this was the explicit motivation for Spark.
- **Iterative graph algorithms** suffer the most — each iteration is a full MR job with HDFS round-trips. [[entities/pregel|Pregel]] was motivated by exactly this weakness; [[entities/systemml|SystemML's]] *piggybacking* combines multiple LOPs into composite jobs to amortize startup cost.

## Related Concepts

- [[concepts/batch-processing]]
- [[concepts/dataflow-engine]]
- [[concepts/declarative-query-language]]
- [[concepts/data-warehouse]]
- [[concepts/column-oriented-storage]]
- [[entities/hdfs]]
- [[entities/apache-spark]]
- [[entities/apache-flink]]
- [[entities/pegasus]]
- [[entities/gbase]]
- [[entities/systemml]]
- [[entities/nscale]]
- [[entities/pregel]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
