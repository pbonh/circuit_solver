---
title: "CAP Theorem"
type: concept
tags: [distributed-systems, consistency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

The CAP theorem (Eric Brewer) states that, in the presence of a network partition, a distributed data system must choose between Consistency (every read sees the most recent committed write) and Availability (every request gets a non-error response). All three (Consistency, Availability, Partition tolerance) cannot be simultaneously guaranteed.

## How It Works

When the network is healthy, systems can be both consistent and available. Under partition, a CP system rejects writes that cannot be propagated to a quorum; an AP system accepts the write at the reachable subset and reconciles later. Most modern databases offer tunable per-request CAP positioning.

## Key Parameters

- Default consistency level.
- Quorum or replication-factor configuration.
- Whether failover is automatic or operator-driven.

## When To Use

CAP framing is a useful first-order lens for trade-off discussions; not a strict design constraint since real systems blend both modes.

## Risks & Pitfalls

- Real-world latency and partial failures break the model further; the PACELC extension is more accurate.
- "AP" or "CP" labels are coarse — most databases tune trade-offs per request.

## Related Concepts

- [[concepts/eventual-consistency]]
- [[concepts/strong-consistency]]
- [[concepts/tunable-consistency]]
- [[concepts/quorum]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
