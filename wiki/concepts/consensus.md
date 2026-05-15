---
title: "Consensus"
type: concept
tags: [distributed-systems, consistency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

Consensus is the problem of getting a set of nodes in a distributed system to agree on a single value (e.g., the next entry in a log, the identity of a leader, the order of an update). It is a foundational primitive underlying replicated state machines, distributed transactions, and leader election.

## How It Works

Fault-tolerant consensus algorithms (Paxos, Multi-Paxos, Raft, ZAB) elect a leader, replicate proposals to a quorum of followers, and commit once a majority acknowledges. They tolerate crash faults and message delays but, by the FLP impossibility result, cannot guarantee bounded-time agreement on a purely asynchronous network. In practice timeouts and retries make consensus achievable.

## Key Parameters

- Quorum size (majority of replicas).
- Election timeout / heartbeat interval.
- Failure model (crash vs. Byzantine).

## When To Use

Leader election, log replication, distributed transactions, configuration management, distributed locks.

## Risks & Pitfalls

- Split-brain when partitions hide a quorum from the original leader.
- Implementation bugs in custom consensus code are notoriously subtle.
- Inability to reach quorum stalls progress.

## Related Concepts

- [[concepts/raft]]
- [[concepts/paxos]]
- [[concepts/quorum]]
- [[concepts/two-generals-problem]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
