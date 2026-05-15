---
title: "Denormalization"
type: concept
tags: [databases, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Denormalization is the deliberate introduction of redundancy into a data model — duplicating fields, embedding child entities, prejoining tables — to optimize for the application's read patterns rather than for data-integrity-driven third normal form.

## How It Works

Instead of writing one canonical record per entity (normalized), an application writes the same value into multiple records or embeds related entities. Reads then need fewer joins (often none). Writes become more complex because all copies must be updated atomically or via eventual consistency.

## Key Parameters

- Update frequency of duplicated fields.
- Application-level merge logic.

## When To Use

Default approach in NoSQL solution-domain modeling, read-heavy workloads, materialized views in relational systems.

## Risks & Pitfalls

- Stale duplicates if updates are missed.
- Disk footprint grows.
- Multi-record updates require transactions or careful retries.

## Related Concepts

- [[concepts/nosql]]
- [[concepts/document-database]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
