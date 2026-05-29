---
title: Redis
type: entity
id: entity-redis
tags:
- database
- key-value
- in-memory
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
---

## Overview

Redis is an in-memory key-value data-structure store first released in 2009. It is widely used as both a distributed cache and a primary data store for use cases that tolerate occasional data loss in exchange for low latency.

## Characteristics

- Single-threaded event loop (multi-threaded networking from v6.0).
- Data structures: strings, linked lists, sets, sorted sets, hashes.
- Persistence via periodic snapshots and/or Append-Only File (AOF) logging.
- Redis Cluster shards across 16,384 hash slots (up to 1,000 nodes).
- Multi/exec "transactions" are serialized but not ACID (no rollback).
- WAIT command provides synchronous replication on demand.
- Proprietary leader-election algorithm.

## Common Strategies

- Cache-aside pattern (most common usage).
- Hash tags to colocate related keys in the same hash slot.
- Master-replica replication for read scaling and failover.

## Related Entities

- [[entities/mongodb]]
- [[entities/dynamodb]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
