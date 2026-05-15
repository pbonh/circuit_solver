---
title: "Leader-Follower Replication"
type: concept
tags: [distributed-systems, replication, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Leader-follower (also primary-secondary, master-slave) replication designates a single replica as the leader for writes; followers apply the leader's update stream asynchronously or synchronously. Reads can be served by the leader (strict freshness) or followers (eventual freshness, scale-out).

## How It Works

The leader serializes incoming writes, appends to its log, and ships log entries to followers. Followers acknowledge receipt; the leader commits once enough acknowledgments arrive. On leader failure, election promotes a follower.

## Key Parameters

- Replication mode (sync, async, semi-sync).
- Read-routing policy (leader vs. follower).
- Failover timeout.

## When To Use

Most relational databases (PostgreSQL, MySQL), document stores (MongoDB), and event logs (Kafka).

## Risks & Pitfalls

- Leader is a write bottleneck.
- Failover requires careful handling of in-flight writes.
- Async followers can lag, producing stale reads.

## Related Concepts

- [[concepts/leaderless-replication]]
- [[concepts/leader-election]]
- [[concepts/read-replica]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
