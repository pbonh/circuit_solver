---
title: PostgreSQL
type: entity
id: entities/postgresql
tags:
- well-established
- relational
- open-source
- database
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
---

## Overview

PostgreSQL is an open-source relational database management system known for standards conformance, extensibility, and a rich feature set including JSON/JSONB, XML, full-text search, geospatial indexing (PostGIS), recursive CTEs, and extension-defined index types (GiST, GIN, BRIN). DDIA uses it as a recurring example to illustrate relational SQL, JSON support, and graph-style queries via WITH RECURSIVE.

## Characteristics

- Mature MVCC-based transactional engine with B-tree primary indexes and write-ahead log.
- Strong support for both relational and document workloads (JSONB).
- PostGIS provides R-tree-style geospatial indexes via the Generalized Search Tree (GiST) framework.
- Supports user-defined functions in multiple languages including JavaScript.
- Schema migrations via ALTER TABLE typically complete in milliseconds (unlike MySQL).

## Common Strategies

- Use JSONB columns for semi-structured data alongside relational schemas.
- Build geospatial systems on PostGIS.
- Replicate using built-in streaming replication; partition large tables natively.
- Adopt foreign data wrappers for federated queries against other systems.

## Related Entities

- [[entities/sql]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
