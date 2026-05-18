---
title: "System Architecture"
type: concept
tags: [architecture, foundational, well-established]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph"
confidence: high
---

## Definition

System architecture is the set of structural and behavioral decisions that define how a system's components are organized, how they interact, and how the system satisfies its quality-attribute requirements. It is the bridge between stakeholder needs and implementable design.

## How It Works

Architecture is expressed through models (C4 diagrams, component diagrams, deployment views), decision records (ADRs), and bounded-context maps. It constrains but does not fully specify implementation: it names the major parts, assigns responsibilities, and defines the protocols that cross part boundaries.

## Key Parameters

- Component decomposition and responsibility assignment
- Communication patterns (synchronous, asynchronous, shared memory, message passing)
- Quality-attribute targets (performance, availability, modifiability, security)
- Technology constraints and integration points

## When To Use

- At project inception, to establish the skeleton around which implementation proceeds
- When a new requirement threatens to violate existing structural assumptions
- During incident post-mortems, when root causes trace back to early structural choices

## Risks & Pitfalls

- Over-specifying architecture before requirements are understood
- Allowing architecture to drift from recorded decisions without ADR supersession
- Conflating architecture with detailed design (premature abstraction)

## Related Concepts

- [[concepts/architectural-decision-record]]
- [[concepts/architecturally-significant-requirement]]
- [[concepts/quality-attributes]]
- [[concepts/bounded-context]]

## Sources

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]]
