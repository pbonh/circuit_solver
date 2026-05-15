---
title: "SQL"
type: entity
tags: [well-established, relational, query-language, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Overview

SQL (Structured Query Language) is the standard declarative query language for relational databases, defined to mirror relational algebra and adopted as the dominant interface for data storage and querying since the mid-1980s. SQL has steadily added features (JSON datatypes, recursive CTEs, XML support, structured datatypes) that close the gap with document and graph models.

## Characteristics

- Declarative: queries describe the desired result; the engine chooses execution.
- Schema-on-write: tables have declared types and constraints, enforced at write time.
- Designed around relations (tables) and joins for many-to-many relationships.
- Supports transactions with ACID guarantees in most relational implementations.
- Recursive common table expressions (WITH RECURSIVE) allow variable-length traversals, enabling graph-style queries (verbose compared to Cypher/SPARQL).
- Hosts MapReduce-style aggregation, materialized views, and full-text search through extensions.

## Common Strategies

- Choose normalization level by query patterns and update frequency.
- Index selectively for read patterns; consider clustered vs covering indexes.
- Use views (virtual or materialized) to encapsulate common queries.
- Apply schema migrations with downtime-aware tools (pt-online-schema-change, gh-ost for MySQL).
- Mix relational and JSON columns when modeling semi-structured data inside an otherwise relational system.

## Related Entities

- [[entities/postgresql]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
