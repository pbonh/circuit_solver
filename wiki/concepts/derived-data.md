---
title: Derived Data
type: claim
id: concepts/derived-data
tags:
- foundational
- well-established
- distributed-systems
- derived-data
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Derived data is data that has been produced by transforming, aggregating, or rewriting some other source data and can be recreated by rerunning the derivation on the source. Caches, search indexes, materialized views, denormalized records, recommendation rankings, and ML training/inference outputs are all derived data. The defining property: losing derived data is not catastrophic because the system of record can regenerate it.

## How It Works

- Identify which dataset is the **system of record** (source of truth).
- Define a deterministic derivation function (SQL projection, batch job, stream transform, ML model training).
- Run the derivation continuously (via stream processing or CDC), periodically (batch), or on demand.
- When the derivation function or schema changes, rerun the derivation to produce a new derived view; run old and new views side by side and gradually migrate consumers.

## Key Parameters

- Freshness vs cost trade-off (how often to refresh).
- Storage format chosen for the access pattern.
- Number of parallel derived views.

## When To Use

For any redundant representation: search indexes, caches, denormalized join tables, OLAP cubes, recommendation models, full-text indexes, geospatial indexes.

## Risks & Pitfalls

- Drift between source and derived view (CDC lag, batch latency).
- Implicit assumptions about derivation order that break under concurrent writes.
- Dual writes (writing to source and derived view directly) cause race conditions; prefer single-source-of-truth + log-based derivation.

## Related Concepts

- [[concepts/system-of-record]]
- [[concepts/change-data-capture]]
- [[concepts/event-sourcing]]
- [[concepts/materialized-view]]
- [[concepts/cqrs]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
