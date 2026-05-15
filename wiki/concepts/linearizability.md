---
title: "Linearizability"
type: concept
tags: [distributed-systems, consistency, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Linearizability is the strongest single-object consistency model. After an operation completes, every subsequent operation observes the new value, ordered by real (wall-clock) time. From the outside, the replicated system behaves as a single, atomic object.

## How It Works

Implementations either funnel writes through a single leader (Raft, Multi-Paxos) or, for distributed reads, contact the leader (or a quorum) to confirm freshness. Spanner uses TrueTime + commit-wait so commit timestamps reflect real-time order with bounded skew.

## Key Parameters

- Whether reads are routed to leader vs. follower.
- Time-source uncertainty bound (TrueTime: ~7 ms).
- Replication topology.

## When To Use

When applications require freshness guarantees across replicas — e.g., distributed locks, leader election, counters that affect business logic.

## Risks & Pitfalls

- Imposes performance overhead (read latency, cross-region delays).
- CAP-bound: cannot have linearizability and availability under partition.

## Related Concepts

- [[concepts/strong-consistency]]
- [[concepts/serializability]]
- [[concepts/truetime]]
- [[concepts/cap-theorem]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
