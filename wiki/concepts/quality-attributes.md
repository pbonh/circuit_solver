---
title: Quality Attributes
type: claim
id: concepts/quality-attributes
tags:
- architecture
- requirements
- foundational
- well-established
created: 2026-05-17
updated: 2026-05-17
sources:
- decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Quality attributes (also called non-functional requirements or -ilities) are the measurable properties of a system that determine how well it performs its function. They include performance, scalability, availability, security, modifiability, testability, and usability. Unlike functional requirements, they rarely appear as user stories but deeply shape architecture.

## How It Works

Quality attributes are specified as scenarios with measurable targets: under what stimulus, under what conditions, what response is required, and how is it measured. Architects trade them against each other — optimizing for performance may reduce modifiability — and record the trade-offs in ADRs.

## Key Parameters

- Stimulus (event that triggers the quality concern)
- Environment (operating conditions)
- Response (system behavior)
- Response measure (quantifiable target)

## When To Use

- When identifying architecturally significant requirements (ASRs)
- During architecture review, to validate that the design meets its quality targets
- When benchmarking or load-testing, to verify that the system behaves as architected

## Risks & Pitfalls

- Vague quality goals ("fast," "secure") that cannot be verified
- Ignoring quality attributes until they become production crises
- Assuming all quality attributes can be simultaneously maximized

## Related Concepts

- [[concepts/architecturally-significant-requirement]]
- [[concepts/system-architecture]]
- [[concepts/architectural-decision-record]]

## Sources

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]]
