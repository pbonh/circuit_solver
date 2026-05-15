---
title: "Fencing Token"
type: concept
tags: [distributed-systems, well-established, concurrency, locks]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

A fencing token is a monotonically increasing number issued by a lock or lease service every time the lock is granted. Downstream resources (storage, services) refuse any request bearing a token lower than the highest one already seen. This prevents an old leaseholder that has been paused or partitioned from corrupting state after a new leaseholder has been chosen.

## How It Works

- The lock service (often ZooKeeper using `zxid` or node `cversion`) returns a token along with the lease.
- The client passes the token on every operation against the protected resource.
- The resource compares the token to its highest-seen value: equal or higher is accepted (and updates the high-water mark); lower is rejected.
- A stale client that wakes up after a GC pause finds its token rejected, preventing the HBase-style data corruption seen in DDIA Figure 8-4.

## Key Parameters

- Token data type (typically 64-bit integer).
- Resource-side high-water mark storage.
- Whether the resource must persist the high-water mark to avoid forgetting after restart.

## When To Use

Wherever distributed locking or leasing is used to protect a shared resource: file storage, database leadership, partition assignment, lease-based caching.

## Risks & Pitfalls

- Only works if the protected resource cooperates; some systems can't be modified to check tokens.
- A malicious client could forge tokens (defeats only inadvertent failure, not Byzantine actors).
- Forgetting to persist the high-water mark on the resource side defeats the protection.
- Token issuance must come from a single authoritative source — a consensus-based lock service.

## Related Concepts

- [[concepts/leader-election]]
- [[concepts/consensus]]
- [[concepts/total-order-broadcast]]
- [[entities/zookeeper]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
