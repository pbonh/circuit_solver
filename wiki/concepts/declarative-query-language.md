---
title: Declarative Query Language
type: claim
id: concepts/declarative-query-language
tags:
- query-languages
- foundational
- well-established
- sql
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A declarative query language specifies what data should be returned and how it should be transformed (filtered, sorted, grouped, aggregated), but not how to compute that result. Examples include SQL, relational algebra, Cypher, SPARQL, Datalog, CSS selectors, and XPath. The query engine (optimizer) chooses execution strategy — indexes, join order, parallelization.

## How It Works

- The query expresses a pattern; the optimizer matches that pattern against available indexes and statistics to produce an execution plan.
- Because the language doesn't constrain execution order, the engine can rewrite storage layout, swap algorithms, or parallelize across cores without breaking queries.
- Imperative APIs (CODASYL navigate, JavaScript DOM manipulation) force the application to specify steps; the engine cannot reorder without breaking semantics.
- Declarative languages tend to be more concise (e.g., 4-line Cypher vs 29-line SQL recursive CTE for the same graph query).
- Hybrids exist: MongoDB's MapReduce is partly imperative; its aggregation pipeline is declarative.

## Key Parameters

- Statistics available to the optimizer (cardinality, distribution).
- Query-hinting facilities for when the optimizer chooses poorly.
- Set of indexes maintained.
- Parallel execution settings (worker count, partitioning).

## When To Use

For any data-access workload where the data layout, indexes, or query patterns may change over time, or where the engine should be free to optimize without forcing query rewrites. Almost always preferable to imperative APIs in storage systems.

## Risks & Pitfalls

- Optimizer surprises: a small data shift can flip plans dramatically.
- Bad queries hide behind clean syntax; expensive operations (Cartesian joins) can be one-line.
- Some query languages (CSS, SPARQL) have edge cases that confuse implementations.
- Declarative does not mean efficient — a poorly written declarative query still scans the world.

## Related Concepts

- [[concepts/relational-model]]
- [[concepts/graph-data-model]]
- [[concepts/mapreduce]]
- [[entities/sql]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
