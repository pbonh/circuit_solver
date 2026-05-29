---
title: Document Database
type: claim
id: concepts/document-database
tags:
- databases
- nosql
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A document database stores semi-structured documents (typically JSON/BSON) under unique keys. Unlike pure KV, the database understands the document's internal structure and can index individual fields.

## How It Works

Documents are organized into logical collections (analogous to tables) without enforced schemas. Queries can match on any indexed field; secondary and compound indexes are common. Examples: MongoDB, Couchbase, Amazon DocumentDB.

## Key Parameters

- Document size limit (MongoDB: 16 MB).
- Index types (single field, compound, geospatial).
- Schema validation rules (optional).

## When To Use

Applications with evolving data shapes, content management, user profiles, catalogs where JSON modeling fits naturally.

## Risks & Pitfalls

- Lack of joins encourages denormalization; updating duplicated data is harder.
- Per-document atomicity but multi-document transactions are expensive.

## Related Concepts

- [[concepts/nosql]]
- [[concepts/denormalization]]
- [[concepts/key-value-store]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
