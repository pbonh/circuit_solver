---
title: Google App Engine
type: entity
id: entities/google-app-engine
tags:
- cloud
- paas
- faas
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
---

## Overview

Google App Engine (GAE) is Google Cloud Platform's managed serverless platform for HTTP-based applications. Released to general availability in 2011, it lets developers deploy code in Go, Java, Python, Node.js, PHP, .NET, and Ruby; GAE manages scaling, load balancing, and capacity.

## Characteristics

- Two environments: standard (tightly managed, fastest scaling) and flexible (Docker on Compute Engine VMs).
- Autoscaling parameters: `target_cpu_utilization`, `target_throughput_utilization`, `max_concurrent_requests`, `min/max_instances`, `max-pending-latency`.
- Cold-start latency 0.5-3 s depending on language.
- Integrates with Firestore, Cloud SQL, Cloud Pub/Sub, and other GCP services.

## Common Strategies

- Set minimum instances > 0 for latency-sensitive paths.
- Tune autoscaling parameters via parametric experiments.

## Related Entities

- [[entities/aws-lambda]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
