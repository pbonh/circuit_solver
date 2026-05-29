---
title: Circuit Breaker
type: claim
id: claim-circuit-breaker
tags:
- distributed-systems
- fault-tolerance
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

A circuit breaker is a client-side guard that monitors errors or latency to a downstream dependency and, when failure rates exceed a threshold, "trips" to fast-fail subsequent calls without sending them. The pattern protects an overwhelmed dependency and gives it time to recover.

## How It Works

The breaker tracks states: CLOSED (normal calls flow), OPEN (calls fail immediately for a timeout), HALF_OPEN (a few trial calls test whether the dependency has recovered). Implementations include Resilience4j (Java), CircuitBreaker (Python), and Hystrix (legacy).

## Key Parameters

- Failure threshold (count or rate).
- Recovery timeout.
- Half-open trial count.

## When To Use

Any time a service calls a remote dependency. Combine with retries (backoff) and bulkheads.

## Risks & Pitfalls

- Too-aggressive thresholds trip on transient blips.
- Without observability you cannot diagnose what tripped the breaker.

## Related Concepts

- [[concepts/cascading-failure]]
- [[concepts/fail-fast]]
- [[concepts/bulkhead-pattern]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
