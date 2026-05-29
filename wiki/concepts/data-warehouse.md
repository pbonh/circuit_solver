---
title: Data Warehouse
type: claim
id: claim-data-warehouse
tags:
- olap
- well-established
- data-warehouse
- batch
- analytics
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

A data warehouse is a separate, read-only database optimized for ad-hoc analytic queries (OLAP), populated by Extract-Transform-Load (ETL) pipelines that consolidate data from many transactional systems. It isolates analytical workloads from OLTP databases so business analysts can run heavy queries without degrading user-facing systems.

## How It Works

- ETL pipelines extract data from OLTP databases (periodic dumps or change streams), transform into an analysis-friendly schema (clean, deduplicate, conform dimensions), and load into the warehouse.
- The warehouse typically uses a relational schema, often a star or snowflake schema with a central fact table surrounded by dimension tables.
- Storage and execution engines are optimized for sequential scans over many rows of few columns — column-oriented storage, compression, vectorized execution.
- Mature commercial warehouses include Teradata, Vertica, SAP HANA, ParAccel; cloud options include Amazon Redshift, Snowflake, BigQuery; open-source SQL-on-Hadoop projects include Hive, Spark SQL, Impala, Presto, Drill, often inspired by Google's Dremel.

## Key Parameters

- ETL cadence (batch nightly vs streaming continuous).
- Schema design (star vs snowflake; dimension granularity).
- Storage format (row vs column; compression codec).
- Materialized aggregates / data cubes maintained.

## When To Use

When an enterprise has multiple OLTP systems whose data analysts need to query together, or when analytic queries are too expensive to run against OLTP databases. Small companies with one OLTP DB and modest data volumes may not need a separate warehouse.

## Risks & Pitfalls

- ETL is brittle, complex, and a source of data-quality issues; modern practice often leans toward ELT (load raw, transform inside the warehouse) or streaming pipelines.
- Schema drift between source systems and warehouse breaks reports silently.
- Materialized cubes accelerate known queries but reduce flexibility for ad-hoc analysis.
- Warehouse data is a snapshot; freshness expectations must match the ETL cadence.

## Related Concepts

- [[concepts/column-oriented-storage]]
- [[concepts/star-schema]]
- [[concepts/oltp-vs-olap]]
- [[concepts/materialized-view]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
