---
title: "Write-Through Cache"
type: concept
tags: [caching, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

A write-through cache synchronously persists writes to the backing data store as part of the cache update. The application writes through the cache; the cache layer guarantees that the durable store has the new value before acknowledging.

## How It Works

The application calls `put(key, value)` on the cache. The cache writes the value to the database (or invokes a configured writer) and only on success updates its own in-memory copy and returns to the caller. Subsequent reads always see a consistent view.

## Key Parameters

- Writer timeout.
- Cache-to-store retry policy.

## When To Use

When reads dominate but consistency between cache and database matters, and write latency can absorb the extra hop.

## Risks & Pitfalls

- Higher write latency than write-behind or cache-aside.
- Cache and store both must be available for writes to succeed.

## Related Concepts

- [[concepts/cache-aside]]
- [[concepts/read-through-cache]]
- [[concepts/write-behind-cache]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
