---
title: Eventual Consistency
type: claim
id: concepts/eventual-consistency
tags:
- distributed-systems
- consistency
- foundational
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

Eventual consistency is a relaxed replica-consistency model that guarantees that, in the absence of further writes, all replicas of a data item will eventually converge to the same value. During the "inconsistency window" different replicas may return different values.

## How It Works

Writes are accepted at one (or any) replica and propagated asynchronously. Anti-entropy repair (Merkle-tree comparisons), read repair, and hinted handoff reconcile divergence over time. Many NoSQL systems offer tunable consistency so a caller can pick stronger guarantees per request via N/W/R parameters.

## Key Parameters

- Inconsistency window length.
- N/W/R values.
- Repair frequency and strategy.

## When To Use

High-volume, geographically distributed systems that prioritize availability over per-request freshness — social feeds, view counts, shopping carts, content delivery.

## Risks & Pitfalls

- Stale reads can confuse users or business logic.
- Concurrent writes can silently lose data under last-writer-wins.
- Application complexity to handle conflicts.

## Related Concepts

- [[concepts/strong-consistency]]
- [[concepts/tunable-consistency]]
- [[concepts/version-vector]]
- [[concepts/last-writer-wins]]
- [[concepts/anti-entropy-repair]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
