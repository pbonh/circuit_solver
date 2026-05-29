---
title: Architecturally Significant Requirement
type: claim
id: concepts/architecturally-significant-requirement
tags:
- requirements
- architecture
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

An architecturally significant requirement (ASR) is a requirement whose effect on a system's structure or quality attributes makes it load-bearing for architectural decisions. ASRs are the trigger for architectural decision records (ADRs); without an ASR, an ADR risks becoming log bloat rather than a justified commitment.

## How It Works

ASRs are identified by asking: "If this requirement changed, would the system's architecture need to change?" Requirements that affect performance, security, scalability, interoperability, or deployment topology are typically architecturally significant. Each ADR must cite the ASR that made the decision necessary.

## Key Parameters

- Structural impact: changes to component boundaries or communication patterns
- Quality-attribute impact: performance, security, availability, modifiability
- Stakeholder priority: business-critical or regulatory requirements

## When To Use

- Before opening any ADR, to ensure the decision is grounded in a real requirement
- During architecture review, to validate that the design addresses the right constraints
- When prioritizing requirements, to separate load-bearing commitments from nice-to-haves

## Risks & Pitfalls

- Inventing ASRs after the fact to justify a pre-selected technical preference
- Conflating every functional requirement with an ASR, leading to ADR proliferation
- Failing to update ASRs when underlying business needs change

## Related Concepts

- [[concepts/architectural-decision-record]]
- [[concepts/system-architecture]]
- [[concepts/quality-attributes]]

## Sources

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]]
