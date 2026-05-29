---
title: Serverless
type: claim
id: claim-serverless
tags:
- cloud
- scalability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

Serverless (function-as-a-service, FaaS) is a cloud execution model in which the platform provisions runtime instances on demand to handle each incoming event. No instances are statically allocated; charges accrue per invocation and per millisecond of execution. AWS Lambda, Google App Engine, Azure Functions, and Apache OpenWhisk are leading examples.

## How It Works

Developers upload a function (with handler, runtime, and dependencies). The platform routes events (HTTP, queue messages, storage triggers) to a runtime instance. Cold starts incur container initialization latency; warm instances are reused. Autoscaling is automatic up to platform-defined burst limits.

## Key Parameters

- Memory allocation (drives CPU).
- Concurrency / provisioned concurrency.
- Timeout per invocation.
- Region-specific burst limit.

## When To Use

Spiky workloads, glue-code APIs, event-driven processing, microservices with intermittent traffic.

## Risks & Pitfalls

- Cold starts add latency.
- Vendor lock-in.
- Cost can balloon for sustained high throughput.

## Related Concepts

- [[concepts/cold-start]]
- [[concepts/provisioned-concurrency]]
- [[concepts/elastic-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
