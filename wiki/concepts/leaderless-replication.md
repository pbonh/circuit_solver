---
title: "Leaderless Replication"
type: concept
tags: [distributed-systems, replication, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

In leaderless replication, any replica can accept writes; the replica acts as a coordinator that fans out the update to peers. The Dynamo paper popularized this style; Cassandra, Riak, and DynamoDB implement variants.

## How It Works

Clients send writes to any replica; that replica updates locally and asynchronously propagates the write to others. Tunable quorum-based reads ensure freshness when needed. Conflicts (concurrent writes to the same key on different coordinators) are resolved with version vectors, last-writer-wins, or CRDTs.

## Key Parameters

- N (replicas), W (write quorum), R (read quorum).
- Conflict-resolution policy.

## When To Use

Highly available, geographically distributed stores prioritizing write throughput.

## Risks & Pitfalls

- Concurrent writes can conflict.
- Stale reads under W=1 or sloppy quorum.

## Related Concepts

- [[concepts/leader-follower-replication]]
- [[concepts/quorum]]
- [[concepts/version-vector]]
- [[concepts/last-writer-wins]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
