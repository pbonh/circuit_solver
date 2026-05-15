---
title: "Shared-Everything Architecture"
type: concept
tags: [distributed-systems, databases, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

In a shared-everything architecture, multiple database engines share the same physical storage (typically via a SAN) and coordinate concurrent access. Oracle Real Application Clusters (RAC) is the canonical example, supporting up to 100 engines that present a single logical database.

## How It Works

A high-speed network connects nodes to shared disks. Proprietary middleware (Clusterware, Cache Fusion) coordinates locks and caches across nodes. Storage hardware provides redundancy (mirroring) to survive disk failures.

## Key Parameters

- SAN bandwidth and IOPS.
- Cluster size.
- Cache-coherence protocol.

## When To Use

Established enterprise applications needing relational scale-out without code changes, willing to pay for proprietary hardware and licensing.

## Risks & Pitfalls

- SAN is an expensive bottleneck and single point of failure.
- Licensing cost is significant.
- Less suited to commodity-hardware cloud deployments.

## Related Concepts

- [[concepts/shared-nothing-architecture]]
- [[concepts/horizontal-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
