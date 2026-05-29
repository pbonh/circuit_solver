---
title: Event-Driven Architecture
type: claim
id: claim-event-driven-architecture
tags:
- distributed-systems
- messaging
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
---

## Definition

In an event-driven architecture, components communicate by emitting and reacting to events — facts about something that has happened. Producers don't know or care which consumers exist; consumers register interest and react asynchronously. The pattern delivers loose coupling and high evolvability.

## How It Works

Events are published to a broker (often Kafka) into a topic or event log. Subscribers process events independently. State changes in one service propagate via events to others; new analyses can be added by replaying the log.

## Key Parameters

- Event schema and versioning.
- Retention policy.
- Subscription/consumer-group strategy.

## When To Use

Cross-service integration, audit trails, analytics pipelines, replicated state.

## Risks & Pitfalls

- Distributed observability is harder than in synchronous workflows.
- Event-schema evolution requires backward-compatibility care.

## Related Concepts

- [[concepts/event-log]]
- [[concepts/publish-subscribe]]
- [[concepts/microservices]]
- [[concepts/choreography]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
