---
title: "Read Replica"
type: concept
tags: [databases, replication, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

A read replica is a secondary database node that asynchronously receives the update stream from a primary and serves read-only queries, scaling out read-heavy workloads while leaving writes on the primary.

## How It Works

The primary streams binlogs or WAL records to one or more secondaries, which apply them to a local copy of the data. Application reads can be directed to any secondary; writes always go to the primary.

## Key Parameters

- Number of replicas.
- Replication lag SLO.
- Synchronous vs. asynchronous mode.

## When To Use

Read-heavy SQL workloads, geographically distributed read traffic, reporting/analytics offloading.

## Risks & Pitfalls

- Stale reads while replication lag is non-zero.
- Failover policies must avoid promoting a behind replica.

## Related Concepts

- [[concepts/leader-follower-replication]]
- [[concepts/horizontal-scaling]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
