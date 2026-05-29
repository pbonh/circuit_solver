---
title: Competing Consumers
type: claim
id: concepts/competing-consumers
tags:
- messaging
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Competing consumers is a messaging pattern in which multiple consumer processes pull from the same queue, each processing a disjoint subset of messages. It scales out message processing without modifying producers or queues.

## How It Works

The broker distributes each message to exactly one of the available consumers using round-robin (RabbitMQ, ActiveMQ) or pull-rate-driven distribution. Adding consumers increases throughput; consumer failures redistribute their unacknowledged messages to peers.

## Key Parameters

- Number of consumers.
- Prefetch count per consumer.
- Acknowledgment mode.

## When To Use

Whenever a single queue's throughput is the bottleneck.

## Risks & Pitfalls

- Ordering is lost across competing consumers.
- Hot keys may saturate one consumer if the broker partitions by key.

## Related Concepts

- [[concepts/message-queue]]
- [[concepts/asynchronous-messaging]]
- [[concepts/consumer-group]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
