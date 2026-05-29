---
title: Cascading Failure
type: claim
id: claim-cascading-failure
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

A cascading failure occurs when a slow or failing component causes back-pressure on its callers, exhausting their resources (threads, memory, connections), which in turn slows their callers, and so on through the call graph — bringing down a whole system.

## How It Works

When a downstream service slows from healthy 50 ms latency to 3000 ms, upstream threads block. With a fixed thread pool, blocked threads quickly exhaust capacity; new requests queue; eventually queues overflow, connections are refused, and clients see errors. Naive retries amplify the load on the already-overwhelmed downstream.

## Key Parameters

- Read timeouts.
- Circuit-breaker trip threshold.
- Bulkhead capacity per API.

## When To Use

Recognize and design against cascading failure in any coupled microservice system.

## Risks & Pitfalls

- Long timeouts deepen the failure.
- Aggressive retries with no backoff worsen the storm.

## Related Concepts

- [[concepts/fail-fast]]
- [[concepts/circuit-breaker]]
- [[concepts/bulkhead-pattern]]
- [[concepts/long-tail-latency]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
