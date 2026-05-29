---
title: Elasticsearch
type: entity
id: entity-elasticsearch
tags:
- well-established
- distributed-systems
- search
- open-source
- derived-data
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
---

## Overview

Elasticsearch is a distributed search and analytics engine built on Apache Lucene. It provides full-text search, structured filtering, geospatial queries, aggregations, and a JSON REST API. Often used as a derived data store fed by CDC pipelines or log-shipping systems, it is a canonical example of a "search index as a derived view" of a source database.

## Characteristics

- Inverted indexes via Lucene segments (SSTable-style merge structure).
- Document-partitioned secondary indexes; reads use scatter-gather.
- Near-real-time indexing with configurable refresh interval.
- Multi-tenancy via indices and shards; replication for fault tolerance.
- Watch/Percolate API allows reverse search: query stored, documents flow past it (used for media monitoring, real-estate alerts, etc.).

## Common Strategies

- Feed via Kafka Connect or Logstash from a source-of-truth database.
- Plan shard count up front — resharding is expensive.
- Use index templates and aliases to manage time-series data and rolling indexes.

## Related Entities

- [[entities/apache-kafka]]
- [[entities/postgresql]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
