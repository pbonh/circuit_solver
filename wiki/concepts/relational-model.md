---
title: Relational Model
type: claim
id: concepts/relational-model
tags:
- foundational
- well-established
- data-modeling
- sql
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

The relational model, proposed by Edgar F. Codd in 1970, organizes data into relations (tables) — unordered collections of tuples (rows). Each row is identified by primary key columns; relationships across tables are expressed by foreign keys. The model hides storage and access-path details behind a declarative interface (SQL), letting the query optimizer choose execution strategy.

## How It Works

- Data lives in flat tables, openly readable in any combination via joins and predicates.
- Any subset of rows can be selected by matching arbitrary conditions; queries are not constrained to predefined access paths.
- Insertion does not require updating link structures (contrast with CODASYL); foreign-key joins are resolved at query time.
- A query optimizer translates declarative queries into execution plans, picking indexes and join orders automatically. Adding an index changes performance, not query text.
- Schemas are enforced on write: every row conforms to a declared schema (schema-on-write), in contrast to document-model schema-on-read.
- Modern relational systems (PostgreSQL, MySQL, DB2) have added JSON and XML support, partially closing the gap with document databases (convergence of models).

## Key Parameters

- Normalization level (1NF, 3NF, etc.) and trade-offs against denormalization.
- Index choice (primary, secondary, covering, clustered) and their write overhead.
- Isolation level for transactions (covered in DDIA Chapter 7).
- ALTER TABLE migration strategy and downtime budget.

## When To Use

When data has regular structure and many-to-many relationships are common; when ad-hoc analytics and reporting are valuable; when ACID transactions, joins, and a mature ecosystem matter; when consistency-on-write is required.

## Risks & Pitfalls

- Object-relational impedance mismatch — ORMs reduce but don't eliminate translation friction.
- Schema migrations can be slow or downtime-inducing (especially MySQL, which copies whole tables on ALTER).
- Highly hierarchical or self-contained-document data is awkward to "shred" into multiple normalized tables.
- Joins on very large tables can be expensive; sharding relational data is hard.

## Related Concepts

- [[concepts/document-model]]
- [[concepts/graph-data-model]]
- [[concepts/declarative-query-language]]
- [[concepts/schema-on-read-vs-schema-on-write]]
- [[entities/sql]]
- [[entities/postgresql]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
