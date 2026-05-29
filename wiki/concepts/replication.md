---
title: Replication
type: claim
id: concepts/replication
tags:
- distributed-systems
- replication
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Replication is the duplication of data or services across multiple nodes so that the system can survive component failures, serve more concurrent reads, and place data closer to users. It is a fundamental scalability and availability primitive in distributed systems.

## How It Works

For stateless services, replication is straightforward — additional replicas behind a load balancer. For data, schemes include leader-follower (one writable primary, asynchronous fan-out to read-only secondaries) and leaderless (any replica can accept writes, with conflicts reconciled). Synchronous replication blocks writes until all replicas confirm; asynchronous trades consistency for write latency.

## Key Parameters

- Replication factor (number of copies).
- Synchronous vs. asynchronous propagation.
- Replica placement (same rack, AZ, region).

## When To Use

Whenever availability, read throughput, or latency-to-user matters more than the storage overhead and consistency complexity of multiple copies.

## Risks & Pitfalls

- Replica consistency is a hard distributed-systems problem.
- Asynchronous replication can lose recent writes on leader failure.
- Inter-region replication adds latency.

## Related Concepts

- [[concepts/leader-follower-replication]]
- [[concepts/leaderless-replication]]
- [[concepts/eventual-consistency]]
- [[concepts/quorum]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
