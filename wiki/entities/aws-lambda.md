---
title: AWS Lambda
type: entity
id: entity-aws-lambda
tags:
- cloud
- serverless
- faas
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
---

## Overview

AWS Lambda is Amazon's serverless function-as-a-service (FaaS) platform. Developers upload code that the platform executes in response to events: HTTP requests via API Gateway, queue messages, S3 events, CloudWatch alarms, and many more.

## Characteristics

- Supported runtimes: Node.js, Python, Ruby, Java, Go, .NET (Docker container images also supported as of 2021).
- Memory: 128 MB - 10 GB; CPU scales with memory.
- Pricing: per-millisecond execution time scaled by memory size.
- Burst concurrency limit varies by region (e.g., 3,000 in us-west-2, 1,000 in eu-central-1) with +500/min after the burst.
- Provisioned concurrency keeps instances warm to avoid cold starts.

## Common Strategies

- Reserved concurrency to isolate functions in the same account.
- Provisioned concurrency for latency-sensitive paths.
- Memory-tuning sweet spot to balance cost vs. latency.
- Integration with API Gateway and Application Load Balancer.

## Related Entities

- [[entities/aws]]
- [[entities/dynamodb]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
