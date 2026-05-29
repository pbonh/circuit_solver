---
title: Read-Through Cache
type: claim
id: concepts/read-through-cache
tags:
- caching
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

In a read-through cache, the application accesses only the cache; when the cache experiences a miss, it transparently invokes a configured loader that fetches data from the system of record and populates itself. The application is shielded from the underlying data store on the read path.

## How It Works

The cache library exposes a `getOrLoad(key)` API and is configured with a loader function or provider interface. On hit it returns the cached value; on miss it invokes the loader, stores the returned value, and returns it. Examples include NCache provider interfaces and Amazon DynamoDB Accelerator (DAX).

## Key Parameters

- Loader timeout.
- TTL per entry.
- Concurrent-load-coalescing strategy (so multiple misses don't all load).

## When To Use

When you want to simplify application logic by hiding the data-store interaction, or to enforce consistent loading semantics across many call sites.

## Risks & Pitfalls

- Adds a dependency from the cache to the database that complicates failure handling.
- Less flexibility than cache-aside in handling per-call-site logic.

## Related Concepts

- [[concepts/cache-aside]]
- [[concepts/write-through-cache]]
- [[concepts/distributed-cache]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
