---
title: "Conflict-Free Replicated Data Types (CRDT)"
type: concept
tags: [distributed-systems, consistency, advanced, emerging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

A Conflict-Free Replicated Data Type (CRDT) is a data structure designed so that concurrent updates from independent replicas can be merged deterministically without coordination. Examples include G-counters, PN-counters, OR-sets, LWW-registers, and CRDT maps.

## How It Works

CRDTs are either state-based (CvRDTs, where replicas exchange whole state and apply an idempotent, commutative merge) or operation-based (CmRDTs, where replicas exchange commutative operations). Either way, after all updates have propagated, replicas converge to the same value without application-level conflict handling.

## Key Parameters

- CRDT variant (counter, set, register, map).
- Merge function.

## When To Use

Collaborative editing (Riak, Redis, Cosmos DB), real-time multi-user counters, shopping cart merges across regions.

## Risks & Pitfalls

- Some operations (e.g., remove-from-set concurrent with add) have surprising semantics.
- State-based CRDTs require careful garbage collection.

## Related Concepts

- [[concepts/version-vector]]
- [[concepts/eventual-consistency]]
- [[concepts/replication]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
