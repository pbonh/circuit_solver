---
title: Microservices
type: claim
id: concepts/microservices
tags:
- microservices
- scalability
- distributed-systems
- foundational
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

Microservices is an architectural style that decomposes an application into many small, independently deployable services, each owning a single business capability and a bounded context. Services communicate over well-defined APIs and can be developed, deployed, and scaled in isolation.

## How It Works

Each microservice is a black box with its own data store, runtime, and team. Services interact via REST/gRPC or asynchronous messaging. An API gateway can front the cluster; workflows are implemented via orchestration or choreography. The "two-pizza rule" refers to team size, not service size.

## Key Parameters

- Service boundaries (typically aligned with DDD bounded contexts).
- Communication style (sync vs. async).
- Independent deployment cadence.

## When To Use

Large applications with multiple teams and varying scalability requirements per subsystem.

## Risks & Pitfalls

- Distributed-system complexity (cascading failures, latency).
- Operational overhead grows with service count.
- Distributed transactions are expensive — design domain boundaries to avoid them.

## Related Concepts

- [[concepts/monolithic-architecture]]
- [[concepts/domain-driven-design]]
- [[concepts/api-gateway]]
- [[concepts/circuit-breaker]]
- [[concepts/bulkhead-pattern]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
