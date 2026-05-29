---
title: Last Writer Wins
type: claim
id: concepts/last-writer-wins
tags:
- distributed-systems
- consistency
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

Last-writer-wins (LWW) is a conflict-resolution policy that, when multiple concurrent updates exist for the same object, retains the one with the most recent timestamp and discards the rest. Simple but lossy.

## How It Works

Each write carries a wall-clock timestamp. On read or replication, the database returns or persists the version with the highest timestamp. Used by default in DynamoDB global tables and many eventually consistent systems.

## Key Parameters

- Timestamp source (wall clock vs. logical clock).
- Tie-breaking rule.

## When To Use

Workloads that can tolerate occasional update loss and require a single deterministic outcome — caching layers, append-only feeds where ordering is best-effort.

## Risks & Pitfalls

- Clock drift means "last" is meaningless when writes are nearly simultaneous on different nodes.
- Concurrent writes from different clients silently lose updates.
- Generally avoided in financial or safety-critical systems.

## Related Concepts

- [[concepts/eventual-consistency]]
- [[concepts/version-vector]]
- [[concepts/clock-drift]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
