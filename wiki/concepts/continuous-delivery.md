---
title: "Continuous Delivery"
type: concept
tags: [well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

Continuous Delivery (CD) is the practice of keeping software in a deployable state at all times via automated build, test, and packaging pipelines. Continuous Deployment goes one step further by automatically pushing every passing commit to production.

## How It Works

Every commit triggers a pipeline: compile, unit test, integration test, security scan, package, smoke test, deploy to staging, deploy to production. Each stage gates progression to the next. Tools include Jenkins, GitHub Actions, GitLab CI, ArgoCD, and Spinnaker.

## Key Parameters

- Pipeline stage definition.
- Test coverage and quality gates.
- Deployment strategy (blue-green, canary, rolling).

## When To Use

The default practice for any team aspiring to scalable, reliable software delivery.

## Risks & Pitfalls

- Fragile pipelines slow everything down.
- Insufficient automated testing pushes bad code to production faster.

## Related Concepts

- [[concepts/devops]]
- [[concepts/infrastructure-as-code]]
- [[concepts/observability]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
