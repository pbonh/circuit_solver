---
title: Sliding Window
type: claim
id: concepts/sliding-window
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

A sliding window is a stream-processing windowing strategy where each output is computed over the most recent W units of data, advancing by a slide interval S < W. Successive windows overlap, so each event contributes to W/S outputs.

## How It Works

In Flink: `SlidingProcessingTimeWindows.of(Time.minutes(10), Time.minutes(5))` produces a result every 5 minutes covering the prior 10-minute window. Useful for moving-average calculations.

## Key Parameters

- Window size W.
- Slide interval S.

## When To Use

Smoothed moving averages, rate calculations, trend detection.

## Risks & Pitfalls

- Memory cost scales with W/S — many overlapping windows.
- Late-arriving events may not fit any window.

## Related Concepts

- [[concepts/tumbling-window]]
- [[concepts/stream-processing]]
- [[concepts/stateful-stream]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
