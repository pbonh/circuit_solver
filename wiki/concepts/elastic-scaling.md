---
title: Elastic Scaling
type: claim
id: claim-elastic-scaling
tags:
- scalability
- distributed-systems
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

Elastic scaling is the dynamic adjustment of resource capacity in response to observed load — automatically adding replicas during spikes and shrinking back during quiet periods to control cost.

## How It Works

A control loop monitors metrics (CPU utilization, queue depth, request rate) against scaling policies and provisions or de-provisions instances accordingly. AWS Auto Scaling Groups, Google Managed Instance Groups, and Kubernetes Horizontal Pod Autoscalers are common implementations.

## Key Parameters

- Min/max instance bounds.
- Scaling-out metric and threshold.
- Scaling-in cooldown / warm-up periods.

## When To Use

Workloads with predictable diurnal patterns or unpredictable bursts (entertainment guides, ticket sales).

## Risks & Pitfalls

- Provisioning lag means new instances take time to absorb load.
- Aggressive scale-in can trigger oscillation.

## Related Concepts

- [[concepts/auto-scaling-group]]
- [[concepts/horizontal-scaling]]
- [[concepts/load-balancing]]
- [[concepts/serverless]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
