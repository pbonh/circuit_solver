---
title: Write-Behind Cache
type: claim
id: claim-write-behind-cache
tags:
- caching
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.65
---

## Definition

A write-behind (or write-back) cache acknowledges writes to the application as soon as the in-memory value is updated, then asynchronously propagates the write to the durable store. Write latency is minimized at the cost of possible data loss on cache failure.

## How It Works

The cache buffers pending writes and a background worker drains them to the database. Many database engines themselves use this pattern internally (buffer cache + WAL flush).

## Key Parameters

- Write-buffer size and flush interval.
- Backpressure threshold.
- Durability/availability trade-off acceptance.

## When To Use

Workloads where write throughput dominates and occasional loss of recent writes is tolerable; ideal for telemetry, metrics, and click-stream-style data.

## Risks & Pitfalls

- Cache crash loses unflushed writes.
- Out-of-order or batched writes complicate downstream consistency.

## Related Concepts

- [[concepts/cache-aside]]
- [[concepts/write-through-cache]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
