---
title: Data Lake
type: claim
id: claim-data-lake
tags:
- databases
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.65
---

## Definition

A data lake is a low-cost, schema-on-read storage repository for historical and heterogeneous data: raw blobs, JSON, CSV, Parquet, database extracts. Unlike a data warehouse, data is stored in its native format and structured only at query time. Common implementations are Apache Hadoop (HDFS), Amazon S3, and Azure Data Lake.

## How It Works

Raw data is ingested into object storage and organized in a hierarchical catalog. Query engines (Athena, Presto, Spark SQL) read and interpret the data on demand. Storage tiers trade access latency for cost.

## Key Parameters

- Storage class / tier.
- Catalog / metadata service.
- Query engine.

## When To Use

Historical archives, regulatory retention, ML training data, exploratory analytics.

## Risks & Pitfalls

- Without governance, becomes a "data swamp" — uncurated and unusable.
- Query performance varies widely by file format and partitioning.

## Related Concepts

- [[concepts/batch-processing]]
- [[concepts/observability]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
