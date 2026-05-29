---
title: Cache-Aside Pattern
type: claim
id: concepts/cache-aside
tags:
- caching
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Cache-aside (also called lazy-loading) is the dominant caching pattern in scalable systems: application code first checks the cache for a key; on a miss it fetches from the database, populates the cache, and returns the value. The cache is not in the write path.

## How It Works

On read, the application does `get(key)`; on miss it queries the system of record, writes the result back into the cache with a TTL, and returns. On write the application invalidates or updates the cached entry. Cache failures degrade performance but do not cause data loss because the database remains authoritative.

## Key Parameters

- TTL per entry.
- Eviction policy (LRU, LFU).
- Update strategy on writes (invalidate vs. refresh).

## When To Use

The default choice for distributed caches such as Redis and memcached. Especially effective when reads vastly outnumber writes.

## Risks & Pitfalls

- Application must remember to invalidate on writes.
- Cold caches produce spikes of database traffic.
- Difficult to maintain consistency across multiple cache entries that depend on the same source data.

## Related Concepts

- [[concepts/distributed-cache]]
- [[concepts/read-through-cache]]
- [[concepts/write-through-cache]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
