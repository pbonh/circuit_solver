---
title: Wide-Column Store
type: claim
id: concepts/wide-column-store
tags:
- databases
- nosql
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A wide-column store extends the key-value model to a two-dimensional hash map: each row is keyed by a row key and contains named columns; columns can vary between rows. Examples include Apache Cassandra, Google Bigtable, and HBase.

## How It Works

Storage is column-family-oriented; columns within a row are sorted by name and can be retrieved in ranges. Cassandra exposes a SQL-like query language (CQL); HBase exposes a CRUD Java API with filters.

## Key Parameters

- Partition key vs. clustering columns.
- Column-family layout.

## When To Use

Time-series data, very high write rates, large sparse data sets, log storage.

## Risks & Pitfalls

- Joins are absent or limited.
- Modeling for query access patterns is essential.

## Related Concepts

- [[concepts/nosql]]
- [[concepts/key-value-store]]
- [[concepts/document-database]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
