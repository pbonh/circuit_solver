---
title: Horizontal Scaling
type: claim
id: concepts/horizontal-scaling
tags:
- distributed-systems
- scalability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Horizontal scaling (scale-out) adds capacity by deploying multiple replicas of a service on additional machines, fronted by a load balancer that distributes requests across them. Total system capacity grows roughly linearly with replica count when services are stateless.

## How It Works

Requests arrive at a load balancer, which selects a target replica using a distribution policy. Each replica is an independent process — typically running the same image — and accesses shared external state (cache, database, message broker) instead of in-process session data. New replicas can be added or removed without code changes; failures of individual replicas degrade capacity but not availability.

## Key Parameters

- Number of replicas in the pool.
- Distribution policy (round-robin, least-connections, weighted).
- Health-check interval.
- Auto-scaling min/max bounds and trigger metrics.

## When To Use

Whenever request volume exceeds the capacity of a single node, when single-node failure cannot be tolerated, or when elasticity (rapid scale-up and scale-down) is desired.

## Risks & Pitfalls

- Requires stateless services or externalized session storage.
- Downstream resources (database, cache, broker) often become the next bottleneck.
- Sticky sessions cause load imbalance.

## Related Concepts

- [[concepts/load-balancing]]
- [[concepts/stateless-service]]
- [[concepts/elastic-scaling]]
- [[concepts/vertical-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
