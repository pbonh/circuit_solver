---
title: OLTP vs OLAP
type: claim
id: concepts/oltp-vs-olap
tags:
- foundational
- well-established
- storage
- data-warehouse
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

OLTP (Online Transaction Processing) and OLAP (Online Analytic Processing) describe two distinct access patterns and the storage engines optimized for each. OLTP serves interactive user-facing workloads with many small reads/writes by key. OLAP serves analyst-facing workloads with fewer but heavier queries that scan large numbers of rows and aggregate them.

## How It Works

OLTP characteristics:
- Small number of records per query, fetched by key.
- Random-access, low-latency writes driven by user input.
- Reads "latest state of data" (current point in time).
- Dataset typically gigabytes to terabytes.
- Storage: row-oriented; indexed by primary and secondary keys; B-trees or LSM-trees.

OLAP characteristics:
- Aggregate over large numbers of records.
- Bulk import (ETL) or event-stream writes.
- Reads "history of events" over time.
- Dataset typically terabytes to petabytes.
- Storage: column-oriented; star/snowflake schemas; vectorized execution.

A common architecture extracts data from OLTP systems and loads it into a separate OLAP data warehouse to isolate the two workloads.

## Key Parameters

- Query latency target (ms for OLTP, seconds-to-minutes for OLAP).
- Read/write ratio.
- Average rows touched per query.
- Storage layout (row vs column).

## When To Use

- OLTP: any user-facing application where individual interactions create or fetch a small number of records.
- OLAP: business intelligence, reporting, ad-hoc analytic exploration, machine learning feature pipelines.

Hybrid (HTAP) systems exist but most production stacks separate the two.

## Risks & Pitfalls

- Running ad-hoc analytics against an OLTP database harms user-facing performance.
- Trying to serve OLTP from a column store is slow (decoding many columns per row).
- The boundary blurs: real-time analytics, streaming OLAP, and "hot" derived views require careful design (covered in DDIA Part III).

## Related Concepts

- [[concepts/data-warehouse]]
- [[concepts/column-oriented-storage]]
- [[concepts/star-schema]]
- [[concepts/b-tree]]
- [[concepts/lsm-tree]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
