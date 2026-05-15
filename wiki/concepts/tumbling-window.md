---
title: "Tumbling Window"
type: concept
tags: [streaming, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

A tumbling window is a stream-processing windowing strategy with disjoint, non-overlapping fixed-size windows. Each event belongs to exactly one window. Contrast with sliding windows, where windows overlap.

## How It Works

Configured by a single window size. Events arriving in [0, W) form one window; events in [W, 2W) form the next; and so on.

## Key Parameters

- Window size.

## When To Use

Reporting metrics on fixed time buckets ("hourly totals"), discrete summaries.

## Risks & Pitfalls

- Edge-of-window effects: an event at time W-1 ms and another at time W+1 ms go to different windows despite being close in time.

## Related Concepts

- [[concepts/sliding-window]]
- [[concepts/stream-processing]]
- [[concepts/stateful-stream]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
