---
title: "Partial Failure"
type: concept
tags: [distributed-systems, fault-tolerance, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

A partial failure is a condition in which some components of a distributed system fail while others continue to operate normally. From a client's perspective, partial failures often manifest as slow or absent responses — the client cannot tell whether the request was lost, the server has crashed, the response is delayed, or the network is congested.

## How It Works

Asynchronous networks deliver messages with variable latency, can drop or duplicate packets, and nodes can crash at any time. When a client does not receive a response within a timeout, it cannot distinguish among many possible failure causes. The typical mitigation is to retry, which requires that mutating server operations be idempotent.

## Key Parameters

- Request timeout values.
- Retry policy (count, backoff strategy).
- Idempotency-key TTL.

## When To Use

Any time a client invokes a remote operation that mutates state. Partial failure must be designed for from the start.

## Risks & Pitfalls

- Naive immediate retries can amplify outages.
- Non-idempotent operations may be applied multiple times.
- Long timeouts cause thread-pool exhaustion and cascading failure.

## Related Concepts

- [[concepts/idempotency]]
- [[concepts/cascading-failure]]
- [[concepts/circuit-breaker]]
- [[concepts/distributed-systems]]

## Sources

- [[summaries/foundations-scalable-systems-03-preface]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
