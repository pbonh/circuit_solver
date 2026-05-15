---
title: "End-to-End Argument"
type: concept
tags: [foundational, well-established, distributed-systems, networking, correctness]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt"]
confidence: high
---

## Definition

The end-to-end argument, articulated by Saltzer, Reed, and Clark in 1984, states that a function can be implemented completely and correctly only with the knowledge and participation of the application standing at the endpoints of a communication system. Low-level mechanisms in the middle may be useful as performance optimizations, but they cannot by themselves provide application-level correctness guarantees.

## How It Works

Classic examples:

- TCP suppresses duplicate packets within a connection; but if a user retries an HTTP request after a timeout, only an application-level operation ID catches the duplicate.
- Ethernet/TLS checksums detect network corruption; but data may still be corrupted on disk or by buggy software at either endpoint. End-to-end checksums (e.g., HDFS block checksums) are needed for that.
- 2PC ensures atomic commit at the database level; but the application still needs end-to-end deduplication if the user might retry across a database session boundary.

In DDIA Example 12-2, a client-generated request UUID is passed through HTTP, application server, and database, and a uniqueness constraint on the request_id column atomically prevents duplicate execution — even if the user retries.

## Key Parameters

- Where to inject the unique identifier (form field, header, HMAC of request).
- Lifetime of the dedup window.
- Whether the endpoint storage enforces the constraint atomically.

## When To Use

For any correctness property that the application cares about — exactly-once execution, integrity, encryption, authentication, deduplication. Treat in-network mechanisms as performance optimizations, not correctness foundations.

## Risks & Pitfalls

- Easy to assume "TCP is reliable" or "the database is transactional" and stop thinking about correctness end-to-end.
- Adds metadata overhead (request IDs everywhere).
- Hard to retrofit onto an existing system that wasn't designed with it.

## Related Concepts

- [[concepts/exactly-once-semantics]]
- [[concepts/idempotency]]
- [[concepts/two-phase-commit]]
- [[concepts/total-order-broadcast]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
