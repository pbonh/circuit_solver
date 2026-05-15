---
title: "Paxos Consensus Algorithm"
type: entity
tags: [well-established, distributed-systems, consensus, algorithm, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: medium
---

## Overview

Paxos is the classic fault-tolerant consensus algorithm, introduced by Leslie Lamport in "The Part-Time Parliament" (1998) and refined in "Paxos Made Simple" (2001). It establishes agreement among a set of nodes on a single value despite crashes and message losses, requiring only that a majority of nodes remain alive. Multi-Paxos extends it to a sequence of decisions, making it equivalent to total order broadcast.

## Characteristics

- Proposers send numbered proposals; acceptors vote on proposals; learners learn the chosen value.
- **Ballot numbers** (ballots) order competing proposals; a proposal succeeds when a quorum of acceptors promises and accepts.
- Two phases per ballot: Prepare (collect promises) and Accept (commit value).
- Multi-Paxos amortizes the Prepare phase across many decisions by keeping a stable leader.
- The foundation of Google Chubby, Spanner, Megastore; influenced ZooKeeper's Zab.
- Famously hard to understand; implementations often have subtle bugs, which motivated Raft.

## Common Strategies

- Use Multi-Paxos with a stable leader to reduce per-decision overhead.
- Combine with Mencius or EPaxos for leaderless variants when geographic latency dominates.
- Build production deployments on existing libraries (etcd/Raft, ZooKeeper/Zab) rather than reimplementing.

## Related Entities

- [[entities/raft]]
- [[entities/zookeeper]]
- [[entities/spanner]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
