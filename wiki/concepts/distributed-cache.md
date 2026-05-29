---
title: Distributed Cache
type: claim
id: concepts/distributed-cache
tags:
- distributed-systems
- caching
- scalability
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

A distributed cache is an in-memory key-value store deployed across multiple nodes and shared by application services. It absorbs read traffic that would otherwise hit a slower database and reduces request latency.

## How It Works

Application code (cache-aside pattern) checks the cache on read; on a miss it queries the database, populates the cache, and returns the result. Writes either invalidate the cached entry or update it. Cache keys are hashed across cache nodes; common implementations are Redis and memcached. TTLs control entry expiration; LRU or LFU eviction reclaims memory when capacity is exceeded.

## Key Parameters

- Cache node count and per-node memory.
- TTL per entry type.
- Eviction policy (LRU, LFU).
- Cache hit/miss target ratio.

## When To Use

Read-heavy workloads where data changes infrequently — catalog data, configuration, computed aggregates, session state.

## Risks & Pitfalls

- Stale reads when entries are not invalidated promptly.
- "Thundering herd" cache misses on hot keys after eviction.
- Cache failure cascading to the database.

## Related Concepts

- [[concepts/cache-aside]]
- [[concepts/read-through-cache]]
- [[concepts/write-through-cache]]
- [[concepts/write-behind-cache]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
