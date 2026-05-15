---
title: "Availability"
type: concept
tags: [distributed-systems, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

Availability is the proportion of time a system responds successfully to requests. It is typically expressed as a percentage ("four nines" = 99.99% = ~52 minutes downtime per year) or in an SLA. Highly available systems are designed to remain operational despite component failures.

## How It Works

Availability is improved by replication, load balancing, health checks, redundancy across availability zones, graceful degradation, and aggressive failure detection plus failover. The CAP theorem links availability to consistency under network partitions.

## Key Parameters

- Target SLA (e.g., 99.9%, 99.99%).
- Replication factor and failover time.
- Health-check timeouts.

## When To Use

Any user-facing system with revenue or safety implications when offline.

## Risks & Pitfalls

- Strong consistency frequently trades against availability under partitions.
- Replication for availability adds consistency complexity.
- Single points of failure undermine multi-9 targets.

## Related Concepts

- [[concepts/replication]]
- [[concepts/cap-theorem]]
- [[concepts/horizontal-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
