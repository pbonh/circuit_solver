---
title: System of Record
type: claim
id: claim-system-of-record
tags:
- foundational
- well-established
- distributed-systems
- derived-data
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.85
---

## Definition

A system of record (a.k.a. source of truth) is the authoritative store of a piece of data: each fact is represented once in normalized form, all derived datasets ultimately trace back to it, and in any discrepancy the value in the system of record is by definition correct. Distinguishing systems of record from derived stores is a powerful architectural lens.

## How It Works

- Pick one place where each piece of data is first written (typically a relational DB, a document store, or an event log).
- All other representations — caches, search indexes, materialized views, warehouses — are derived from the system of record via well-defined transformations.
- Writes to the system propagate to derived stores via change data capture or event sourcing.
- Recovery from data corruption involves rebuilding derived stores from the system of record.

## Key Parameters

- Granularity (which fields belong in which system of record).
- Durability and replication of the system of record.
- Schema evolution policy.

## When To Use

Always for any non-trivial data system. Being explicit about which store is authoritative clarifies failure recovery, schema migrations, and consistency models.

## Risks & Pitfalls

- Multiple systems of record for the same data (dual writes) cause conflicts.
- Sometimes the "system of record" lives in an external organization (regulatory filings, payment processors); plan for asynchronous reconciliation.
- Tightly coupling the system-of-record schema to derived schemas hampers evolution.

## Related Concepts

- [[concepts/derived-data]]
- [[concepts/event-sourcing]]
- [[concepts/change-data-capture]]
- [[concepts/data-warehouse]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
