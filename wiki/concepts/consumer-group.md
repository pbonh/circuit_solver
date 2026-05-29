---
title: Consumer Group
type: claim
id: concepts/consumer-group
tags:
- streaming
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Kafka consumer group is a set of cooperating consumer processes that share consumption of a topic. Each topic partition is assigned to exactly one consumer in the group, so the group as a whole sees every message once, while different consumers in the group see disjoint subsets.

## How It Works

A broker-side group coordinator tracks group membership and assigns partitions to consumers. When a consumer joins, leaves, or a partition is added, a rebalance reassigns partitions to consumers. Kafka exposes the CooperativeStickyAssignor to minimize reassignment churn.

## Key Parameters

- Group id.
- Number of consumers (up to partition count for full parallelism).
- Rebalance protocol.

## When To Use

Whenever you want to scale message consumption horizontally while preserving per-key ordering.

## Risks & Pitfalls

- Stop-the-world rebalances under naive assignors disrupt processing.
- More consumers than partitions leave some consumers idle.

## Related Concepts

- [[concepts/topic-partition]]
- [[concepts/competing-consumers]]
- [[concepts/event-log]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
