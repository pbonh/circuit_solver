---
title: "Raft"
type: concept
tags: [distributed-systems, consensus, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources:
  - "raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"
  - "raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"
confidence: high
---

## Definition

Raft is a leader-based, crash-fault-tolerant [[concepts/consensus]] algorithm designed by Diego Ongaro and John Ousterhout (Ongaro, "In Search of an Understandable Consensus Algorithm," 2014) explicitly to be easier to understand and implement than [[concepts/paxos]]. It implements fault-tolerant total-order broadcast over a fixed cluster by electing a strong leader that drives a replicated log; a committed log entry is preserved across all future leaders (the *log matching* safety property). Raft is the consensus algorithm used by [[entities/etcd]], Consul, [[entities/cockroachdb]], TiKV, [[entities/neo4j]], [[entities/yugabytedb]], Hazelcast, RethinkDB, and many other modern systems.

## How It Works

A cluster of an odd number of nodes (typically 3 or 5) maintains a replicated log. One node is the leader for a numbered *term*; followers receive periodic `AppendEntries` RPCs as both heartbeats and log-replication carriers. The leader accepts client requests, appends them to its log, replicates to followers, and commits an entry once a majority (quorum) has acknowledged.

- **Strong leader.** All client requests go to the current leader; followers redirect.
- **Term numbers** identify the current leadership era; a higher-term leader supersedes any earlier one.
- **Leader election.** Follower election timers are randomized to avoid split votes. On timeout a follower becomes a candidate, increments its term, and requests votes; a candidate with *all previously committed entries* and a majority of votes wins.
- **Log replication.** Leader appends entries, replicates via `AppendEntries`, commits once a majority acknowledges; followers replay committed entries against the local state machine.
- **Membership changes** are handled by the joint-consensus mechanism so that the cluster can transition configurations without losing quorum.
- **Snapshotting** bounds log size: a node periodically snapshots state and truncates the prefix of its log.

## Key Parameters

- Cluster size (odd, typically 3–7); tolerates ⌊(n − 1) / 2⌋ failures.
- Heartbeat interval (often 100–500 ms) — drives liveness and election timeouts.
- Election-timeout range — randomized window above the heartbeat interval to avoid split votes.

## When To Use

- Replicated state machines that need linearizable consistency.
- Distributed coordination and metadata services (etcd, Consul, ZooKeeper-style use cases).
- Consensus-backed databases requiring strong leader semantics and predictable performance.

## Risks & Pitfalls

- Implementation subtleties around log truncation, snapshotting, and the joint-consensus membership change are common bug sources.
- Network partitions can cause repeated leader elections; the *pre-vote* optimization avoids disruption from a partitioned candidate that has incremented its term.
- A single strong leader bounds throughput; geographic deployments may benefit from leaderless variants like EPaxos / Mencius (see [[concepts/paxos]]).

## Related Concepts

- [[concepts/consensus]]
- [[concepts/paxos]]
- [[concepts/leader-election]]
- [[concepts/quorum]]
- [[concepts/total-order-broadcast]]
- [[concepts/replication]]
- [[entities/etcd]]
- [[entities/zookeeper]]
- [[entities/cockroachdb]]
- [[entities/yugabytedb]]
- [[entities/neo4j]]
- [[entities/spanner]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
- [[summaries/ddia-04-part-ii-distributed-data]]
