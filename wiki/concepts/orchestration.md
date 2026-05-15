---
title: "Orchestration"
type: concept
tags: [microservices, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

In microservice workflows, orchestration centralizes the control logic in a single component (often a dedicated orchestrator microservice) that calls downstream services in sequence to satisfy a business operation. Contrast with choreography, where peers coordinate via events.

## How It Works

The orchestrator (e.g., Netflix Conductor) executes a workflow definition, invoking each step, handling failures, and aggregating results. State of the workflow is centralized, making it easier to monitor and reason about.

## Key Parameters

- Workflow language / DSL.
- Retry and compensation policies.

## When To Use

Long-running workflows, complex business processes that span many services, scenarios where central monitoring matters.

## Risks & Pitfalls

- Orchestrator becomes a bottleneck or single point of failure.
- Couples downstream services into one workflow.

## Related Concepts

- [[concepts/choreography]]
- [[concepts/microservices]]
- [[concepts/event-driven-architecture]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
