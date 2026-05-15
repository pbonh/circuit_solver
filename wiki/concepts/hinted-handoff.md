---
title: "Hinted Handoff"
type: concept
tags: [distributed-systems, replication, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

Hinted handoff is the mechanism by which a temporary holder of a write (selected during a sloppy quorum) forwards the data to the original home replica once that replica becomes reachable again.

## How It Works

The substitute node stores the write with a "hint" recording the intended home replica. A background process retries delivery until the home replica accepts the write, at which point the local copy is discarded.

## Key Parameters

- Hint retry interval.
- Hint expiration.
- Maximum hint queue size per substitute.

## When To Use

In leaderless or Dynamo-style replicated stores prioritizing write availability.

## Risks & Pitfalls

- Hints can pile up during long outages.
- The original replica may receive hints out of order; vector clocks or version vectors are needed to resolve.

## Related Concepts

- [[concepts/sloppy-quorum]]
- [[concepts/quorum]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
