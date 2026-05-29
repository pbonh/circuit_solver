---
title: Schema-on-Read vs Schema-on-Write
type: claim
id: concepts/schema-on-read-vs-schema-on-write
tags:
- data-modeling
- well-established
- schema-evolution
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

Schema-on-write (traditional relational approach) requires data to conform to a declared schema before being stored; the database rejects nonconforming writes. Schema-on-read (used by JSON/document databases, schemaless stores, Hadoop) accepts arbitrary structure on write and interprets it only when read; the schema is implicit in the reader's code. The analogy is static vs dynamic type-checking in programming languages.

## How It Works

Schema-on-write:
- Schema declared in DDL (CREATE TABLE, ALTER TABLE).
- Database enforces types, constraints, and defaults at insert/update time.
- Migrations (ALTER TABLE, UPDATE) are explicit and may require downtime.

Schema-on-read:
- Documents are written with whatever fields the application chooses.
- Reading code handles missing fields or new fields by branching (e.g., `if (!user.first_name) user.first_name = user.name.split(" ")[0]`).
- Multiple "schema versions" can coexist in storage indefinitely (data outlives code).

## Key Parameters

- Tolerance for heterogeneous data in the same collection.
- Migration cost and downtime budget.
- Cost of bad data slipping through (catch-at-write vs catch-at-read).

## When To Use

- Schema-on-write: when data is homogeneous, when constraints catch bugs early, when downstream consumers expect a fixed structure.
- Schema-on-read: when the structure is heterogeneous, externally controlled, or changes frequently; when migrating large amounts of data is impractical.

## Risks & Pitfalls

- Schema-on-read shifts the burden of schema validation to every reader — easy to drift.
- Schema-on-write migrations on large tables can be slow (especially MySQL ALTER TABLE).
- The phrase "schemaless" is misleading; there is always an implicit schema in reading code.
- Mixed schema versions in one store can be hard to query consistently.

## Related Concepts

- [[concepts/relational-model]]
- [[concepts/document-model]]
- [[concepts/schema-evolution]]
- [[concepts/backward-and-forward-compatibility]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
