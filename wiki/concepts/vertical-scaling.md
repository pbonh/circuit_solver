---
title: Vertical Scaling
type: claim
id: concepts/vertical-scaling
tags:
- distributed-systems
- scalability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Vertical scaling (scale-up) increases system capacity by deploying onto a more powerful machine — more CPUs, more memory, faster disks — without modifying application code. It is the simplest first response to a growing load.

## How It Works

Migrate the running service or database to a larger instance class (e.g., AWS `t3.xlarge` -> `t3.2xlarge`). The OS and runtime exploit the additional hardware; application code usually requires no change. Database engines in particular benefit dramatically from large memory caches and many cores.

## Key Parameters

- Target instance type (vCPUs, memory, disk IOPS).
- Migration window (especially for stateful systems).

## When To Use

For early-stage systems, modest growth, single-node-only databases, or when adopting a distributed architecture would add disproportionate complexity.

## Risks & Pitfalls

- Hardware cost grows exponentially with size.
- Hits a ceiling beyond which no single machine suffices.
- A scaled-up single node is still a single point of failure.
- Single-threaded code does not benefit from extra cores (Amdahl's law).

## Related Concepts

- [[concepts/horizontal-scaling]]
- [[concepts/amdahls-law]]
- [[concepts/scalability]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
