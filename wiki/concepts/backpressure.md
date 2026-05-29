---
title: Backpressure
type: claim
id: concepts/backpressure
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
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Backpressure is the propagation of consumption-rate constraints upstream so that producers slow down or buffer when consumers cannot keep up. Without backpressure, producers overwhelm consumers, leading to queue overflow or memory exhaustion.

## How It Works

Reactive Streams, RxJava, and similar libraries expose explicit pull/request semantics from downstream. In messaging systems, brokers throttle producers when memory or disk thresholds are reached (RabbitMQ at ~40% memory).

## Key Parameters

- Threshold at which backpressure engages.
- Producer-pause / pause-buffer strategy.

## When To Use

Streaming pipelines, async messaging, anywhere producer and consumer rates can diverge.

## Risks & Pitfalls

- Without backpressure, unbounded queues mask overload.
- Excessively eager backpressure starves throughput.

## Related Concepts

- [[concepts/cascading-failure]]
- [[concepts/throttling]]
- [[concepts/asynchronous-messaging]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
