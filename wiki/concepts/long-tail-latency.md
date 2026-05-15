---
title: "Long-Tail Latency"
type: concept
tags: [distributed-systems, performance, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: high
---

## Definition

Long-tail latency refers to the distribution of response times in real workloads where a small fraction of requests take dramatically longer than the median. Tail percentiles (P95, P99, P99.9) capture this much better than averages.

## How It Works

Long tails arise from garbage collection pauses, database contention, network drops, context switching, page faults, and noisy-neighbor effects. P99 can be 10-20x the median; at scale 1% of requests is still millions per day.

## Key Parameters

- Percentile of interest (P95, P99, P99.9).
- Per-percentile SLO.

## When To Use

Any time you're measuring scalable system performance. Mean response time hides tail issues.

## Risks & Pitfalls

- Averages give a false sense of security.
- Many backend calls compose multiplicatively into worse user-perceived latency.

## Related Concepts

- [[concepts/cascading-failure]]
- [[concepts/fail-fast]]
- [[concepts/observability]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
