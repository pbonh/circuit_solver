---
title: "Leader Election"
type: concept
tags: [distributed-systems, well-established, consensus]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Leader election is the procedure by which a distributed cluster agrees on exactly one node to act as the leader (primary, master) for a partition, replica set, or service. Correct election prevents split brain — two nodes both believing they are leader — which causes diverging writes and data loss.

## How It Works

- Detect that the current leader has failed, typically via missed heartbeats and a timeout. There is no perfect failure detector in partially synchronous systems.
- Run a consensus round (Paxos, Raft, Zab, VSR) to elect a new leader. Each candidate proposes itself with a higher epoch/term number; nodes vote for at most one candidate per term.
- The winning candidate must have the most up-to-date log to avoid losing committed writes.
- The new leader takes over only after it confirms it has a quorum of votes; old leaders (if alive but unreachable) are fenced off via increasing epoch numbers.
- Coordination services like ZooKeeper, etcd, and Consul provide leader-election primitives (ephemeral nodes plus sequence numbers; leases).

## Key Parameters

- Election timeout (and randomization to break ties).
- Heartbeat interval.
- Fencing token / epoch number issuance.
- Pre-vote phase to avoid disruptive churn under partitions.

## When To Use

For any single-writer, single-coordinator role: single-leader DB replicas, job schedulers, partition coordinators, distributed locks.

## Risks & Pitfalls

- Network partitions can cause repeated leader churn; Raft has known edge cases with a flaky link.
- Without fencing tokens, an old leader recovering from a pause can write stale data (HBase bug, Figure 8-4).
- Aggressive timeouts cause unnecessary failovers; conservative timeouts increase outage windows.
- A demoted leader that doesn't realize it has lost office can corrupt the system if downstream services don't verify fencing.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/quorum]]
- [[concepts/fencing-token]]
- [[concepts/replication]]
- [[entities/zookeeper]]
- [[entities/etcd]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
