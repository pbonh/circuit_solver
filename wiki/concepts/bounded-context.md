---
title: Bounded Context
type: claim
id: claim-bounded-context
tags:
- foundational
- domain-driven-design
- well-established
created: 2026-05-17
updated: 2026-05-17
sources: []
confidence:
  base: 0.45
---

## Definition

A bounded context is a central pattern in Domain-Driven Design (DDD) that defines an explicit boundary around a domain model. Within the boundary, a ubiquitous language and a consistent set of domain invariants are maintained. Terms may have different meanings outside the boundary.

## How It Works

Bounded contexts are identified by looking for cohesive subdomains where the same terms are used consistently. Each context gets its own model, and relationships between contexts are governed by integration patterns such as shared-kernel, customer-supplier, or anticorruption-layer.

## Key Parameters

- Ubiquitous language inside the boundary.
- Invariants that must hold within the model.
- Integration patterns with adjacent contexts.

## When To Use

When a domain is large enough that the same word means different things in different parts, or when subsystems are maintained by different teams with different expertise.

## Risks & Pitfalls

- Drawing the boundary too small creates excessive mapping overhead.
- Drawing it too large allows language drift and anemic models.
- Skipping explicit context maps leads to "false cognate" bugs at integration boundaries.

## Related Concepts

- [[concepts/domain-driven-design]]
- [[concepts/context-map]]
- [[concepts/false-cognate]]

## Sources

- (Stub — created during Strategy workflow for circuit-solver. Expand with raw source on DDD.)
