---
title: "In-Sync Replica (ISR)"
type: concept
tags: [streaming, replication, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

In Kafka, the In-Sync Replica (ISR) set is the dynamically maintained list of follower replicas that are sufficiently caught up with the leader to be considered eligible for promotion. Only ISR members can become leaders on failover.

## How It Works

The broker tracks each follower's fetch progress; a follower that falls more than a configured threshold behind the leader is removed from the ISR. Producers using `acks=all` wait until all ISR members persist a write before acknowledging. `min.insync.replicas` defines the minimum ISR for a write to succeed.

## Key Parameters

- replica.lag.time.max.ms.
- min.insync.replicas.
- Replication factor.

## When To Use

Always configure ISR-related parameters for production Kafka.

## Risks & Pitfalls

- ISR shrinking to a single replica is a red-flag operational alert.
- Setting `min.insync.replicas` higher than ISR size will block writes.

## Related Concepts

- [[concepts/topic-partition]]
- [[concepts/leader-follower-replication]]
- [[concepts/leader-election]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
