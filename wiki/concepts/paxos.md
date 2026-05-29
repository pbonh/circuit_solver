---
title: Paxos
type: claim
id: concepts/paxos
tags:
- distributed-systems
- consensus
- algorithm
- advanced
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

Paxos is Leslie Lamport's classic fault-tolerant [[concepts/consensus]] algorithm, introduced in "The Part-Time Parliament" (1998) and refined in "Paxos Made Simple" (2001). It establishes agreement among a set of nodes on a single value despite crashes and message losses, requiring only that a majority of nodes remain alive. *Multi-Paxos* extends the single-decision protocol to a sequence of decisions, making it equivalent to total-order broadcast. The original is leaderless and notoriously hard to implement correctly; the Multi-Paxos variant introduces a stable leader and behaves much like [[concepts/raft]] in practice. Paxos is the foundation of Google Chubby, [[entities/spanner|Spanner]], and Megastore; it influenced [[entities/zookeeper|ZooKeeper's]] Zab protocol.

## How It Works

The protocol assigns three logical roles — *proposers*, *acceptors*, and *learners* — although in practice each node typically plays all three.

- **Ballot numbering.** Each proposal carries a globally-unique, monotonically increasing ballot (proposal) number that totally orders competing proposals.
- **Phase 1 — Prepare.** A proposer sends a `Prepare(n)` to a quorum of acceptors. Each acceptor replies with a `Promise` not to accept any ballot lower than n, along with the highest-numbered value it has already accepted (if any).
- **Phase 2 — Accept.** If the proposer collects promises from a quorum, it issues `Accept(n, v)` for value v (the value attached to the highest-numbered prior acceptance it saw, if any; otherwise its own proposal). When a quorum of acceptors accept, v is *chosen*.
- **Multi-Paxos.** Amortizes Phase 1 across many decisions by electing a stable leader that runs Phase 1 once per term and then only Phase 2 per decision.

## Key Parameters

- Proposal / ballot numbering scheme (must be globally unique and totally ordered).
- Quorum size — strict majority of acceptors for both phases; reads typically also require a quorum.

## When To Use

- Long-lived replicated state machines requiring strong consistency: Spanner, Chubby, Megastore.
- When proven safety in the presence of arbitrary message reorderings is required.
- Geographically distributed deployments where leaderless variants such as **EPaxos** or **Mencius** can outperform a single-leader protocol like Raft by avoiding cross-region leader traffic.

## Risks & Pitfalls

- **Implementation complexity.** Paxos has historically been a source of subtle bugs; this is what motivated [[concepts/raft]].
- **Dueling proposers.** Without a stable leader, two proposers can keep pre-empting each other's ballots and stall progress indefinitely (a liveness failure that is consistent with FLP impossibility — Paxos guarantees only safety, not liveness, without leader stabilization).
- For most production deployments the right answer is to use an existing library (etcd/Raft, ZooKeeper/Zab) rather than reimplement Paxos.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/raft]]
- [[concepts/leader-election]]
- [[concepts/quorum]]
- [[concepts/total-order-broadcast]]
- [[entities/zookeeper]]
- [[entities/spanner]]
- [[entities/etcd]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
- [[summaries/ddia-04-part-ii-distributed-data]]
