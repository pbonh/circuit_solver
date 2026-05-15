---
title: "Raft"
type: concept
tags: [distributed-systems, consensus, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Raft is a leader-based, fault-tolerant consensus algorithm designed by Ongaro and Ousterhout (2013) for understandability and ease of implementation. It is used by etcd, Neo4j, YugabyteDB, Hazelcast, and many others.

## How It Works

A cluster of an odd number of nodes (typically 3 or 5) maintains a replicated log. One node is the leader for a numbered "term"; followers receive periodic AppendEntries heartbeats. The leader accepts client requests, replicates entries to followers, and commits once a majority has acknowledged. On leader failure, follower election timers (randomized) trigger candidacy: a node increments term and requests votes; a candidate with all previously committed entries can win.

## Key Parameters

- Cluster size (odd, 3-7 typical).
- Heartbeat interval (300-500 ms).
- Election-timeout range.

## When To Use

Replicated state machines, distributed coordination, consensus-backed databases.

## Risks & Pitfalls

- Implementation subtleties around log truncation and snapshotting.
- Network partitions can cause repeated leader elections.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/paxos]]
- [[concepts/leader-election]]
- [[concepts/quorum]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
