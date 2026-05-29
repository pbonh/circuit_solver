---
title: Clock Drift
type: claim
id: concepts/clock-drift
tags:
- distributed-systems
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Clock drift is the divergence over time of the local clock on a node from "true" time, due to physical effects such as temperature and voltage variation. Typical drifts are 10-20 seconds per day; this makes naive cross-node timestamp comparison meaningless.

## How It Works

Each node has at least two clocks: a time-of-day clock (resettable, can jump backward after NTP correction) and a monotonic clock (only moves forward, useful for measuring elapsed time on a single node). Time services such as NTP, Chrony, and Amazon Time Sync Service periodically correct the time-of-day clock; Spanner's TrueTime uses GPS + atomic clocks to bound uncertainty.

## Key Parameters

- Drift rate (seconds/day).
- Re-sync interval.
- Bounded-skew guarantee (TrueTime: ~7 ms).

## When To Use

Always relevant when designing distributed algorithms; particularly important for replica consistency and event ordering.

## Risks & Pitfalls

- Using wall-clock timestamps to order events across nodes silently corrupts ordering.
- Time can jump backward after NTP correction; intervals can come out negative.
- Bounded-uncertainty time (TrueTime) is rare; most systems must use logical clocks instead.

## Related Concepts

- [[concepts/ntp]]
- [[concepts/logical-clock]]
- [[concepts/truetime]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
