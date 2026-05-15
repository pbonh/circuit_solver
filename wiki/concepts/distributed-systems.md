---
title: "Distributed Systems"
type: concept
tags: [distributed-systems, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

A distributed system is a collection of independent computational nodes that cooperate over a network to provide a service to clients. The defining characteristics are that nodes communicate by message passing, components can fail independently, network latencies are variable, and clocks across nodes are not synchronized.

## How It Works

Each node runs its own processes and accesses its own local memory and storage; coordination is by explicit message exchange. To clients, the collection ideally looks like a single service. Achieving this requires solutions to the universal problems of partial failure, concurrency, time/ordering, and consensus on shared state.

## Key Parameters

- Network topology (LAN/WAN/cellular) and corresponding latency/bandwidth budgets.
- Failure model (crash, omission, Byzantine).
- Consistency model (eventual, strong, linearizable).
- Replication factor and partitioning scheme.

## When To Use

When a single machine cannot meet capacity, latency, geographic-reach, or availability requirements. Avoid distribution if a single node suffices — it adds significant complexity.

## Risks & Pitfalls

- Partial failure is the norm, not an exception.
- "Fallacies of distributed computing" — assuming the network is reliable, latency is zero, bandwidth is infinite, the topology never changes, etc.
- Cross-node clock comparison is unreliable.
- Coordination overhead can dominate at scale.

## Related Concepts

- [[concepts/partial-failure]]
- [[concepts/consensus]]
- [[concepts/replication]]
- [[concepts/scalability]]

## Sources

- [[summaries/foundations-scalable-systems-00-cover]]
- [[summaries/foundations-scalable-systems-03-preface]]
