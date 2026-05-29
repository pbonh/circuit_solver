---
title: Choreography
type: claim
id: claim-choreography
tags:
- microservices
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.65
---

## Definition

In microservice workflows, choreography is the peer-to-peer alternative to orchestration: services react to events emitted by others without a central coordinator. Control logic is decentralized; each service knows only its own piece of the workflow.

## How It Works

Services publish domain events to a broker; downstream services subscribe to events of interest, perform their step, and emit further events. Communication is typically asynchronous publish-subscribe.

## Key Parameters

- Event schema and versioning.
- Subscription pattern.

## When To Use

Loosely coupled workflows with relatively independent stages; event-driven architectures.

## Risks & Pitfalls

- Difficult to trace and monitor end-to-end progress.
- Implicit dependencies between services hidden in event subscriptions.

## Related Concepts

- [[concepts/orchestration]]
- [[concepts/event-driven-architecture]]
- [[concepts/publish-subscribe]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
