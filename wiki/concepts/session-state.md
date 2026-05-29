---
title: Session State
type: claim
id: claim-session-state
tags:
- distributed-systems
- microservices
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

Session state is the data a service retains about a single client's interactions across multiple requests — e.g., login identity, shopping cart contents, or workflow position. In scalable systems it should be externalized rather than held in service memory.

## How It Works

Upon login or first interaction the service creates a session identifier and stores associated data in a distributed cache (Redis, memcached) or database. The identifier is returned to the client (commonly as an HTTP cookie or JWT). Subsequent requests carry the identifier so that any replica can look up the state.

## Key Parameters

- Session TTL and idle-timeout.
- Storage backend (cache vs. durable store).
- Session-key randomness and signing.

## When To Use

Any time per-client state must persist beyond a single request and the service tier scales horizontally.

## Risks & Pitfalls

- Holding session state in-process prevents horizontal scaling.
- Long TTLs accumulate stale data and inflate storage cost.
- Session fixation and replay attacks require careful key handling.

## Related Concepts

- [[concepts/stateless-service]]
- [[concepts/distributed-cache]]
- [[concepts/http-caching]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
