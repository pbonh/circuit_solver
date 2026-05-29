---
title: Scalability
type: claim
id: concepts/scalability
tags:
- distributed-systems
- foundational
- well-established
- performance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Scalability describes a system's ability to cope with increased load. It is not a binary label ("X is scalable") but a question: "If load grows along axis Y, what options do we have for handling the growth?" Discussing scalability requires choosing load parameters (requests/sec, ratio of reads to writes, fan-out, hit rate) and performance metrics (throughput, response-time distribution).

## How It Works

- Describe the load on the system with a few specific load parameters that match the workload (e.g., Twitter's fan-out distribution: 4.6k posts/sec but 345k home-timeline writes/sec because each post can be delivered to many followers, with celebrities as outliers handled hybrid).
- Describe performance two ways: how does performance degrade if load grows at fixed resources, and how must resources grow to keep performance constant.
- Use response-time percentiles (p50, p95, p99, p999) rather than means, because tail latency drives user-perceived experience and is amplified when multiple backend calls compose.
- Architecturally: scale up (vertical, one big machine), scale out (horizontal, shared-nothing distribution), or a pragmatic mixture; some workloads can be elastic (auto-scaling), others manually scaled for operational predictability.

## Key Parameters

- Load parameters appropriate to the workload (fan-out, requests/sec, dataset size, write/read ratio, peak vs average).
- Response-time SLOs/SLAs at chosen percentiles.
- Replication factor and partitioning scheme (covered in Part II).
- Elasticity threshold and policy for auto-scaling.

## When To Use

Whenever growth in users, data volume, or traffic is anticipated. Premature scaling is a form of premature optimization in early-stage products, but mature systems usually need to rethink architecture on every order-of-magnitude load increase.

## Risks & Pitfalls

- Optimizing for the wrong load parameter is at best wasted effort, at worst counterproductive.
- Mean response time hides tail latency; head-of-line blocking and queueing delays dominate p99/p999.
- Naive load testing where the client waits for each response artificially shortens queues and produces misleading results.
- Stateful data systems are much harder to scale out than stateless services.

## Related Concepts

- [[concepts/reliability]]
- [[concepts/maintainability]]
- [[concepts/response-time-percentiles]]
- [[concepts/fault-tolerance]]

## Sources

- [[summaries/ddia-02-preface]]
- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/foundations-scalable-systems-00-cover]]
- [[summaries/foundations-scalable-systems-03-preface]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
