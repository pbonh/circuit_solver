---
title: "Anti-Entropy Repair"
type: concept
tags: [distributed-systems, replication, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

Anti-entropy repair is the family of mechanisms that reconcile divergent replicas in an eventually consistent database. Active (read) repair runs on each read; passive repair runs periodically in the background, often using Merkle trees.

## How It Works

Read repair compares replica values (or hashes) during reads and asynchronously updates stale replicas with the latest value. Passive repair scans all data using Merkle-tree comparison: nodes exchange root hashes, descending into the tree only where hashes differ, until divergent leaves are identified and the stale data is updated.

## Key Parameters

- Read-repair probability (fraction of reads that trigger comparison).
- Passive-repair frequency and concurrency.
- Merkle-tree depth and chunking strategy.

## When To Use

Mandatory in any eventually consistent system to prevent silent divergence.

## Risks & Pitfalls

- Repair traffic can rival production traffic.
- Aggressive repair amplifies disk I/O.

## Related Concepts

- [[concepts/merkle-tree]]
- [[concepts/eventual-consistency]]
- [[concepts/replication]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
