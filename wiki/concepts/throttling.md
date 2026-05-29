---
title: Throttling
type: claim
id: concepts/throttling
tags:
- distributed-systems
- fault-tolerance
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

Throttling (rate limiting) is the practice of explicitly capping request rates or concurrent in-flight requests to a service, returning HTTP 429 or 503 once the cap is reached. It protects upstream resources from overload and enforces fair-use policies.

## How It Works

Common algorithms: token bucket, leaky bucket, sliding window. Implementations live in API gateways, load balancers (HAProxy, NGINX), or in-application logic that tracks an in-flight counter or response-time metric.

## Key Parameters

- Allowed requests per second.
- Burst capacity.
- Per-client vs. global quota.

## When To Use

Any externally facing API, especially in API gateways and microservices.

## Risks & Pitfalls

- Throttled clients without retry/backoff retry-storm the limiter.
- Choosing the wrong limit can blackhole legitimate traffic.

## Related Concepts

- [[concepts/fail-fast]]
- [[concepts/api-gateway]]
- [[concepts/backpressure]]
- [[concepts/circuit-breaker]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
