---
title: Auto Scaling Group
type: claim
id: claim-auto-scaling-group
tags:
- scalability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.65
---

## Definition

An Auto Scaling Group (ASG) is the AWS abstraction for managing a fleet of identical compute instances behind a load balancer with elastic scaling policies. Equivalent concepts exist in GCP (Managed Instance Groups), Azure (VM Scale Sets), and Kubernetes (Deployments + HPA).

## How It Works

The ASG holds a launch template plus min/max/desired instance counts. Scaling policies — schedule-based or metric-based — adjust the desired count over time. Health checks remove and replace failed instances automatically.

## Key Parameters

- Min, max, desired instance count.
- Warm-up and cooldown periods.
- Scaling metric (CPU utilization, queue depth, custom CloudWatch metric).

## When To Use

Any workload that benefits from elasticity and runs on stateless compute instances.

## Risks & Pitfalls

- Instances do not start immediately; cold-start lag is significant.
- Stateful workloads need careful pre-warming and draining.

## Related Concepts

- [[concepts/elastic-scaling]]
- [[concepts/horizontal-scaling]]
- [[concepts/load-balancing]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
