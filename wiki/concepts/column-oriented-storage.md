---
title: Column-Oriented Storage
type: claim
id: concepts/column-oriented-storage
tags:
- storage
- well-established
- olap
- data-warehouse
- performance
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

Column-oriented storage lays out a table's data column-by-column on disk rather than row-by-row. Each column is stored in its own file (or contiguous region), with rows in identical order across columns so the kth entry in every column file belongs to the kth row. This layout is foundational for OLAP/data-warehouse workloads that scan many rows of only a few columns. Examples: C-Store, Vertica, Parquet, Apache ORC, BigQuery/Dremel.

## How It Works

- Only columns referenced by the query are read from disk, drastically reducing I/O for wide fact tables (hundreds of columns) where queries touch only a handful.
- Compression is highly effective because consecutive values in a column are often repetitive: bitmap encoding (one bitmap per distinct value), run-length encoding, dictionary encoding, delta encoding.
- Sorted column data enables efficient bitmap AND/OR operations to evaluate predicates without materializing rows.
- Vectorized processing operates on compressed column chunks fitting in CPU L1 cache, using SIMD and tight loops without per-row function calls.
- Sort order: choose primary sort key based on common predicates (e.g., date_key); secondary sort keys give grouping. C-Store/Vertica store the same data sorted multiple ways across replicas.
- Writes are handled LSM-style: buffered in memory (row- or column-oriented) and merged into the column files in bulk; queries combine the buffer with the on-disk columns.

## Key Parameters

- Sort columns and their order.
- Compression codec per column (chosen by column data type and distribution).
- Block size for compressed segments.
- Number of replicated copies with different sort orders.

## When To Use

For analytics, business intelligence, and reporting workloads with wide tables and read-mostly access patterns. Modern data warehouses (Redshift, BigQuery, Snowflake, Vertica, ClickHouse) are essentially all column stores.

## Risks & Pitfalls

- Update-in-place is hard with compressed columns; writes batch through an LSM layer.
- Loading and decoding many columns to reconstruct a full row is expensive — bad for OLTP-style point reads.
- "Column families" in Cassandra/HBase are not column-oriented in this sense; they still store rows together.
- Adding a column may require rewriting all data depending on the format.

## Related Concepts

- [[concepts/data-warehouse]]
- [[concepts/star-schema]]
- [[concepts/oltp-vs-olap]]
- [[concepts/materialized-view]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
