---
title: Lambda Architecture
type: claim
id: concepts/lambda-architecture
tags:
- streaming
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The Lambda architecture combines batch and stream processing in a single system. A batch layer periodically processes large data sets to produce authoritative results; a speed layer processes the same events in real-time to provide low-latency approximate results until the batch layer catches up. A serving layer answers queries from both layers.

## How It Works

Events flow into both layers simultaneously. The batch layer (historically Hadoop) produces accurate but stale views; the speed layer (Storm) produces low-latency but possibly approximate views. The serving layer merges them.

## Key Parameters

- Batch interval.
- Stream-processing latency.
- Storage technology for serving layer.

## When To Use

Mostly historical at this point; the simpler Kappa architecture has displaced Lambda in many new systems.

## Risks & Pitfalls

- Logic must be implemented twice (batch and streaming) and kept in sync.
- Operational complexity of two parallel pipelines.

## Related Concepts

- [[concepts/kappa-architecture]]
- [[concepts/stream-processing]]
- [[concepts/batch-processing]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
