---
title: "Document Model"
type: concept
tags: [data-modeling, well-established, nosql, json]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

The document model stores data as self-contained records (documents) — typically JSON, XML, or BSON — where related items can be nested within a parent rather than split across separate tables. It is a modern revival of the hierarchical model of the 1960s/70s. Document databases include MongoDB, RethinkDB, CouchDB, and Espresso.

## How It Works

- A document holds tree-structured data (one-to-many relationships expressed by nesting).
- Locality: an entire document is stored contiguously, so a single fetch can return all data needed to render an entity (e.g., a resume's positions, education, contact info).
- References between documents use document IDs and are resolved at query time, similar to foreign keys; joins are weakly supported (none in MongoDB historically, predeclared views in CouchDB, supported in RethinkDB).
- Most document databases are schema-on-read: the database does not enforce structure, but application code typically assumes one.
- Migrations are often handled by branching in application code (if old field absent, derive from new).

## Key Parameters

- Maximum recommended document size (writes typically rewrite the entire document).
- Indexing strategy (primary key, secondary indexes inside nested fields).
- Replication and sharding key choice.

## When To Use

When the data is naturally a self-contained tree, joins are rare, and the application loads the whole tree at once. When schema flexibility (heterogeneous documents) is needed, or when external systems define the schema and may change it.

## Risks & Pitfalls

- Many-to-many relationships and joins are awkward; emulating joins in application code is slower and complicates consistency.
- Highly interconnected data becomes "schemaless mud" over time.
- Updates rewrite whole documents; large or grow-unboundedly documents hurt performance.
- Schema-on-read shifts the burden of schema validation to readers; mistakes manifest at read time.

## Related Concepts

- [[concepts/relational-model]]
- [[concepts/graph-data-model]]
- [[concepts/schema-on-read-vs-schema-on-write]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
