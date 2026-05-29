---
title: Stateless Service
type: claim
id: concepts/stateless-service
tags:
- distributed-systems
- microservices
- scalability
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

A stateless service is one whose API implementations retain no in-process knowledge about prior client interactions. Each request carries all the information the service needs to process it; any required session state lives in an external store such as a distributed cache or database.

## How It Works

Sessions are tracked via tokens or cookies that index session data stored in Redis, memcached, or a transactional database. Because any replica can process any request, load balancers can distribute traffic freely and failures cause only the affected in-flight requests to be retried — no in-memory state is lost.

## Key Parameters

- External session store choice (Redis, memcached, database).
- Session TTL.
- Token signing and validation strategy.

## When To Use

Whenever the service must scale horizontally or tolerate replica failures gracefully — i.e., almost always for internet-facing tiers.

## Risks & Pitfalls

- Latency cost of every-request session-store lookup.
- Session-store failure becomes a new single point of failure.
- Frameworks that encourage in-memory sessions can lead to accidental statefulness.

## Related Concepts

- [[concepts/session-state]]
- [[concepts/horizontal-scaling]]
- [[concepts/load-balancing]]
- [[concepts/distributed-cache]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
