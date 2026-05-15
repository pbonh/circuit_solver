---
title: "Fail Fast"
type: concept
tags: [distributed-systems, fault-tolerance, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: high
---

## Definition

Fail fast is the design principle that, when a request cannot be served within a reasonable time, the system should return an error immediately rather than waiting. This frees resources (threads, connections) for other work and prevents cascading failure.

## How It Works

Two main techniques: client-side read timeouts on outbound requests (usually set near P99 of normal latency) and server-side throttling that rejects requests exceeding a configured load with HTTP 503. Combine with default/fallback responses to mask transient failures.

## Key Parameters

- Read-timeout value.
- Throttling threshold.
- Fallback strategy.

## When To Use

In every microservice with downstream dependencies; in API gateways and load balancers.

## Risks & Pitfalls

- Timeouts too aggressive cause spurious failures on slow but valid responses.
- Without circuit breakers, fail-fast still ties up the dependent endpoint.

## Related Concepts

- [[concepts/circuit-breaker]]
- [[concepts/throttling]]
- [[concepts/cascading-failure]]
- [[concepts/long-tail-latency]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
