---
title: Architectural Decision Record
type: claim
id: claim-architectural-decision-record
tags:
- architecture
- documentation
- well-established
created: 2026-05-17
updated: 2026-05-17
sources:
- decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph
confidence:
  base: 0.85
---

## Definition

An architectural decision record (ADR) is a document that captures an important architectural decision made along with its context and consequences. ADRs create a persistent log of why the system is structured the way it is, making the rationale discoverable long after the original decision-makers have moved on.

## How It Works

Each ADR is numbered sequentially and includes: the status of the decision (proposed, accepted, deprecated, superseded), the context that triggered it, the decision itself stated as a commitment, and the consequences (positive, negative, and neutral). Once accepted, an ADR is write-once; to change a decision, a new ADR supersedes the old one.

## Key Parameters

- Status lifecycle: proposed → accepted → deprecated / superseded
- One decision per ADR — no bundling
- Must cite an architecturally significant requirement (ASR) in `## Context`
- Monotonic numbering across the wiki, never reused

## When To Use

- Before encoding a structural commitment in code
- When a decision affects component boundaries, communication patterns, or quality attributes
- During architecture review, to validate that load-bearing choices are recorded

## Risks & Pitfalls

- ADRs without an ASR citation become log bloat rather than justified commitments
- Bundling multiple decisions in one record makes supersession messy
- Failing to update status when a decision is deprecated or superseded

## Related Concepts

- [[concepts/architecturally-significant-requirement]]
- [[concepts/system-architecture]]
- [[concepts/quality-attributes]]

## Sources

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]]
