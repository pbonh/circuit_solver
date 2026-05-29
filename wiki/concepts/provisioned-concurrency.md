---
title: Provisioned Concurrency
type: claim
id: concepts/provisioned-concurrency
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
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Provisioned concurrency is an AWS Lambda (and equivalents elsewhere) feature that keeps a configured number of runtime instances pre-warmed and ready, eliminating cold-start latency for that many concurrent invocations.

## How It Works

You specify a target instance count. AWS pre-initializes that many containers, keeping them warm and routing requests to them first. Bursts beyond the provisioned count overflow to on-demand instances, which still incur cold starts.

## Key Parameters

- Provisioned-instance count.
- Schedule (can be ramped up via Application Auto Scaling).

## When To Use

Latency-sensitive serverless APIs with predictable baseline load.

## Risks & Pitfalls

- Charges accrue continuously regardless of usage.
- Under-provisioning still causes cold starts during spikes.

## Related Concepts

- [[concepts/cold-start]]
- [[concepts/serverless]]
- [[concepts/elastic-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
