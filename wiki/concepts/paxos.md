---
title: "Paxos"
type: concept
tags: [distributed-systems, consensus, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

Paxos is Leslie Lamport's classic fault-tolerant consensus algorithm. The original is leaderless and notoriously hard to implement; the Multi-Paxos variant introduces a stable leader and behaves much like Raft in practice. Used in Google Cloud Spanner and many internal Google systems.

## How It Works

Proposers send numbered proposals to acceptors; an acceptor responds with the highest-numbered proposal it has accepted. If a proposer gets responses from a quorum, it can issue an accept request, which becomes the agreed value when a quorum accepts. Multi-Paxos amortizes this by having a stable leader handle many decisions per term.

## Key Parameters

- Proposal numbering.
- Quorum size.

## When To Use

Long-lived state machines requiring strong consistency: Spanner, Chubby, Megastore.

## Risks & Pitfalls

- Implementation complexity has historically led to bugs.
- Without a stable leader, dueling proposers can stall progress.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/raft]]
- [[concepts/leader-election]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
