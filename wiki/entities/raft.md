---
title: "Raft Consensus Algorithm"
type: entity
tags: [well-established, distributed-systems, consensus, algorithm]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: medium
---

## Overview

Raft is a consensus algorithm designed by Diego Ongaro and John Ousterhout (2014) explicitly to be easier to understand than Paxos. It implements fault-tolerant total order broadcast over a fixed cluster, electing a leader that drives a replicated log. Raft is the consensus algorithm used by etcd, Consul, CockroachDB, TiKV, RethinkDB, and many other modern systems.

## Characteristics

- Strong leader: all client requests go to the current leader, who appends to its log and replicates to followers.
- **Term numbers** identify the current leader; higher-term leaders supersede earlier ones.
- Leader election uses randomized timeouts to avoid split votes; a candidate needs a majority quorum.
- Log replication: leader appends entries, replicates with `AppendEntries` RPCs, commits once a majority acknowledges.
- Safety property: a committed entry is preserved across all future leaders (log matching property).
- Membership changes via the joint-consensus mechanism.

## Common Strategies

- Cluster sizes of 3 or 5 nodes; tolerates `(n-1)/2` failures.
- Pre-vote optimization to avoid disruption from a partitioned candidate.
- Snapshotting to bound log size.

## Related Entities

- [[entities/paxos]]
- [[entities/zookeeper]]
- [[entities/etcd]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
