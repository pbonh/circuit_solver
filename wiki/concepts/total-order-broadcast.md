---
title: "Total Order Broadcast"
type: concept
tags: [distributed-systems, well-established, consensus, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Total order broadcast (a.k.a. atomic broadcast) is a protocol for delivering messages to multiple nodes such that two safety properties hold: **reliable delivery** (every non-faulty node receives every message) and **totally ordered delivery** (all nodes see messages in identical order). It is provably equivalent in power to consensus: solving one solves the other.

## How It Works

- Implemented by consensus algorithms (Paxos/Multi-Paxos, Raft, Zab, VSR), which sequence successive proposals into an append-only log delivered to every replica.
- ZooKeeper and etcd expose total order broadcast under the hood; both build linearizable storage on top.
- Once a message is delivered, its position in the order is fixed — no later retroactive insertion.
- Applications: state-machine replication (every replica applies the same writes in the same order), serializable distributed transactions, fencing-token issuance, deterministic logs for CDC.
- Linearizable atomic compare-and-set can be built from total order broadcast (append claim, wait for delivery, check who got there first) and vice versa.

## Key Parameters

- Log retention policy.
- Throughput vs latency tuning (batching, pipelining).
- Membership change protocol.

## When To Use

As the abstraction underlying replicated state machines, distributed locks with fencing, leader election, configuration stores, and exactly-once message processing.

## Risks & Pitfalls

- Throughput is bounded by the slowest of a quorum; tail latency dominates.
- All operations serialized through the log; not suitable for high-write-rate datasets without sharding.
- Implementing it directly is famously hard — use ZooKeeper, etcd, or a battle-tested library.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/linearizability]]
- [[concepts/leader-election]]
- [[concepts/fencing-token]]
- [[concepts/lamport-timestamp]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
