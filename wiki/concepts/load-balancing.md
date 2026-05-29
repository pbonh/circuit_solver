---
title: Load Balancing
type: claim
id: claim-load-balancing
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
  base: 0.85
---

## Definition

A load balancer is an intermediary (typically a reverse proxy) that distributes incoming client requests across a pool of service replicas, with the goal of utilizing each replica's capacity evenly while improving aggregate throughput, response time, and availability.

## How It Works

Layer-4 (network) balancers route at the TCP/UDP level using NAT, making decisions based on packet headers; they are fast but feature-light. Layer-7 (application) balancers reassemble HTTP requests and route based on URLs, headers, or body, supporting richer policies at slightly higher overhead. Standard distribution policies include round-robin, least-connections, weighted, and content-based. Periodic health checks remove unhealthy targets from rotation.

## Key Parameters

- Distribution policy (round-robin, least-connections, header/verb, weighted).
- Health-check interval and failure threshold.
- Connection draining timeouts.
- Session-affinity / sticky-session settings.

## When To Use

In front of any replicated, stateless service tier. Also used to terminate TLS, hide backend IPs, and provide a single ingress for elasticity.

## Risks & Pitfalls

- Sticky sessions create load imbalance over time.
- A poorly tuned balancer becomes a bottleneck or single point of failure.
- Health checks that are too lax route traffic to degraded nodes.

## Related Concepts

- [[concepts/horizontal-scaling]]
- [[concepts/stateless-service]]
- [[concepts/elastic-scaling]]
- [[concepts/auto-scaling-group]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
