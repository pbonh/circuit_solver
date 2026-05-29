---
title: CQRS (Command Query Responsibility Segregation)
type: claim
id: claim-cqrs
tags:
- well-established
- derived-data
- architecture
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.65
---

## Definition

CQRS — Command Query Responsibility Segregation — is an architectural pattern that separates the model used to write data (commands) from the model used to read data (queries). Writes go to a write-optimized data structure (often an event log), and reads come from one or more read-optimized derived views materialized from those writes. Coined by Greg Young, building on Bertrand Meyer's earlier Command/Query Separation principle.

## How It Works

- Commands mutate state; they go through validation and produce events in an append-only log.
- Multiple read views can be materialized from the same event log, each optimized for a different query pattern.
- Read views can use any storage technology — relational tables for reports, search indexes for text queries, key-value stores for fast lookup.
- Updates to read views may be asynchronous (eventually consistent) or synchronous (linearizable but with coordination cost).

## Key Parameters

- Number and shape of derived views.
- Consistency model (sync vs async update).
- Snapshot strategy for read views.

## When To Use

When read and write workloads have very different access patterns; when multiple representations of the same data are needed; when event sourcing is already adopted; when scaling read throughput independently of write throughput is valuable.

## Risks & Pitfalls

- Additional complexity vs a single relational model.
- Eventual consistency between command side and query side requires UX consideration (read-your-writes).
- Operational overhead of multiple read stores.

## Related Concepts

- [[concepts/event-sourcing]]
- [[concepts/materialized-view]]
- [[concepts/derived-data]]
- [[concepts/change-data-capture]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
