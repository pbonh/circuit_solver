---
title: "Two-Phase Commit (2PC)"
type: concept
tags: [distributed-systems, consistency, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Two-phase commit (2PC) is the classical distributed transaction protocol. A coordinator drives a vote: in the prepare phase each participant durably promises to commit (or abort); in the resolve phase the coordinator broadcasts the final outcome.

## How It Works

1. **Prepare**: coordinator asks each participant if it can commit. Participants persist the proposed changes and reply YES or NO.
2. **Resolve**: if all replied YES, the coordinator broadcasts COMMIT and each participant finalizes. If any replied NO (or timed out), the coordinator broadcasts ABORT.

If the coordinator crashes between phases, participants block holding locks until the coordinator recovers and consults its transaction log. This blocking weakness motivates fault-tolerant variants (3PC, consensus-backed coordinators in Spanner).

## Key Parameters

- Prepare timeout.
- Transaction-log durability.
- Coordinator-recovery procedure.

## When To Use

Distributed ACID transactions across heterogeneous resources (JTA/JTS, XA), multi-partition commits in distributed SQL databases.

## Risks & Pitfalls

- Coordinator failure causes participants to block.
- Holding locks across network round-trips degrades throughput.
- Cascading failure when overloaded.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/acid-transactions]]
- [[concepts/raft]]
- [[concepts/paxos]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
