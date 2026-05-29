---
title: MongoDB
type: entity
id: entities/mongodb
tags:
- well-established
- nosql
- document-database
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
---

## Overview

MongoDB is a widely used document-oriented database that stores data as BSON (binary JSON) documents in collections. It rose to prominence as part of the NoSQL movement and is one of DDIA's recurring examples of the document model, schema-on-read tradeoffs, MapReduce querying, and aggregation pipelines.

## Characteristics

- BSON document storage with collection-level grouping; no enforced schema (though optional validation exists).
- Originally lacked joins (later added via $lookup); document references are resolved client-side or with helpers.
- Provides both a MapReduce interface and a declarative aggregation pipeline ($match, $group, $sum, etc.) — the moral of which is that "a NoSQL system may find itself accidentally reinventing SQL."
- Updates rewrite entire documents; design guidance keeps documents small and avoids growth in place.
- Supports sharding and replica sets for horizontal scale and fault tolerance.

## Common Strategies

- Model tree-structured, self-contained entities as single documents.
- Use references plus client-side joins for many-to-many; or denormalize with caution.
- Apply schema-on-read with explicit version checks in application code rather than ALTER-like migrations.
- Use aggregation pipelines instead of MapReduce for analytic queries.

## Related Entities

- [[entities/sql]]
- [[entities/postgresql]]
- [[entities/cassandra]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
