---
title: Publish-Subscribe
type: claim
id: claim-publish-subscribe
tags:
- messaging
- distributed-systems
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

Publish-subscribe (pub/sub) is a messaging pattern where producers publish messages to a topic and the broker delivers each message to every active subscriber, decoupling senders from receivers.

## How It Works

Subscribers register interest in a topic. The broker maintains per-subscriber delivery state, holding messages until each subscriber has consumed and acknowledged them. Subscribers can be added or removed dynamically without producer awareness.

## Key Parameters

- Subscriber acknowledgment timeout.
- Per-subscriber retention.
- Topic filtering / routing rules.

## When To Use

Event-driven architectures, integration of multiple downstream consumers from a single event stream, fan-out notification systems.

## Risks & Pitfalls

- Slow subscribers can balloon broker storage.
- Broker becomes the central scaling concern.

## Related Concepts

- [[concepts/asynchronous-messaging]]
- [[concepts/message-broker]]
- [[concepts/event-driven-architecture]]
- [[concepts/topic-partition]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
