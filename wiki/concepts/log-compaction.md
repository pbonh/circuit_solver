---
title: "Log Compaction"
type: concept
tags: [streaming, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

Log compaction is a retention policy for Kafka topics in which only the latest event per key is preserved. Older values for the same key are eventually deleted. A null value acts as a tombstone marking the key for deletion.

## How It Works

A periodic compaction task scans each partition, retaining only the most recent record for each key. Useful for materializing "latest state" snapshots from an immutable log.

## Key Parameters

- Compaction frequency.
- Tombstone retention period.

## When To Use

Topics that represent the latest state of an entity (e.g., user profile updates), not all historical events.

## Risks & Pitfalls

- Loses history of intermediate values.
- Right-to-be-forgotten use cases must rely on tombstones plus compaction.

## Related Concepts

- [[concepts/event-log]]
- [[concepts/topic-partition]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
