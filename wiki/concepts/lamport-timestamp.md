---
title: Lamport Timestamp
type: claim
id: concepts/lamport-timestamp
tags:
- distributed-systems
- well-established
- ordering
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Lamport timestamp, proposed by Leslie Lamport in 1978, is a pair `(counter, node_id)` that provides a total ordering of events in a distributed system that is consistent with causality. Two events `A` and `B` satisfy `A < B` if `A.counter < B.counter`, or `A.counter == B.counter` and `A.node_id < B.node_id`.

## How It Works

- Each node maintains a local counter.
- On each local event the counter increments by 1.
- Every message (request, response, replication update) carries the sender's current counter.
- On receiving a message with counter `c`, the receiver sets its counter to `max(local, c) + 1`.
- The result: any causally preceding event has a strictly smaller timestamp; concurrent events may have either order but are totally ordered by the (counter, node) pair.

## Key Parameters

- Counter width (32 vs 64 bit).
- Node-id assignment strategy.

## When To Use

For ordering log entries, version-stamping records, generating monotonic IDs across nodes when only total ordering — not real-time — is needed. Compact compared to version vectors.

## Risks & Pitfalls

- Lamport timestamps lose the ability to detect concurrency (unlike version vectors); you cannot tell if `A` and `B` were concurrent or causally related from the timestamps alone.
- Total ordering after the fact is not enough to enforce real-time uniqueness constraints; you need total-order broadcast for that.
- Counter desynchronization across nodes is fine for ordering but does not measure elapsed time.

## Related Concepts

- [[concepts/version-vector]]
- [[concepts/total-order-broadcast]]
- [[concepts/causal-consistency]]
- [[concepts/clock-skew]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
