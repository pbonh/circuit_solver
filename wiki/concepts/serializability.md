---
title: Serializability
type: claim
id: claim-serializability
tags:
- databases
- consistency
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

Serializability is the strongest standard isolation level for database transactions. It guarantees that the outcome of a set of concurrent transactions is equivalent to some serial (one-at-a-time) execution of those transactions. It is the "C" in ACID and the "C" in transactional consistency.

## How It Works

Engines implement serializability via strict two-phase locking, serializable snapshot isolation, or single-threaded execution (VoltDB). Combined with linearizability for single-object reads, you get strict serializability, the strongest distributed-systems consistency model.

## Key Parameters

- Concurrency control mechanism (locking vs. SSI).
- Conflict detection granularity.

## When To Use

When application invariants depend on inter-transaction ordering that weaker isolation levels would violate (e.g., write-skew anomalies).

## Risks & Pitfalls

- Throughput is significantly lower than with weaker isolation.
- Distributed serializability requires consensus protocols.

## Related Concepts

- [[concepts/linearizability]]
- [[concepts/acid-transactions]]
- [[concepts/strong-consistency]]
- [[concepts/snapshot-isolation]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
