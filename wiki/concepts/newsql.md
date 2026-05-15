---
title: "NewSQL"
type: concept
tags: [databases, distributed-systems, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

NewSQL is the class of distributed databases that preserve the relational model, SQL, and ACID transactions while delivering horizontal scaling, high availability, and global distribution. Often called "Distributed SQL" today.

## How It Works

NewSQL engines combine sharded relational storage with consensus-based replication (Paxos, Raft) and distributed transactions (2PC coordinated by a Paxos group). Examples include Google Cloud Spanner, CockroachDB, YugabyteDB, and VoltDB.

## Key Parameters

- Consensus algorithm.
- Transaction-coordinator topology.
- Time source (TrueTime, NTP, HLC).

## When To Use

Applications that need SQL and ACID at internet scale.

## Risks & Pitfalls

- Higher write latency than NoSQL.
- Schema migrations at scale are still difficult.

## Related Concepts

- [[concepts/nosql]]
- [[concepts/strong-consistency]]
- [[concepts/raft]]
- [[concepts/two-phase-commit]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
