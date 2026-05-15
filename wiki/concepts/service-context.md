---
title: "Service Context"
type: concept
tags: [simulation, modeling, soa, soc-devs, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/19-15-cloud-system-simulation-modeling.txt"]
confidence: low
---

## Definition

A Service Context is the per-invocation state-tracking object maintained inside a SOC-DEVS software service. It holds the context ID, operation ID, request message, execution time, and the list of jobs sent to the hardware. The service context lets the service handle multiple concurrent invocations of the same or different operations.

## How It Works

When the service receives a request, it creates a service context and submits a job (parameterized by required CPU cycles and memory) to the assigned hardware. The hardware processes the job and returns completion; the service then looks up the matching context and emits the response message. Once an operation is completed, the associated context is removed.

## Key Parameters

- Context ID
- Operation ID
- Request message reference
- Execution time
- Pending and completed job lists

## When To Use

- Handling multiple concurrent invocations within one service
- Tracking long-running operations across hardware queueing
- Composing primitive services into composite services

## Risks & Pitfalls

- Context-list growth under bursty load can exhaust memory
- Context lifecycle errors leak state
- Per-context ID generation must be unique across runs

## Related Concepts

- [[concepts/soc-devs]]
- [[concepts/soad-framework]]
- [[concepts/atomic-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-19-15-cloud-system-simulation-modeling]]
