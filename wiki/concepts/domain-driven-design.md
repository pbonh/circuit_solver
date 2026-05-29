---
title: Domain-Driven Design
type: claim
id: concepts/domain-driven-design
tags:
- foundational
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

Domain-Driven Design (DDD), introduced by Eric Evans (2003), is a software design approach that aligns code structure with the business domain. Core concepts include the ubiquitous language, bounded contexts, aggregates, entities, value objects, and domain events.

## How It Works

Stakeholders and engineers collaboratively model the domain in shared language. Bounded contexts identify cohesive subdomains; each becomes a strong candidate for a microservice boundary. Aggregates encapsulate consistency invariants; domain events propagate state changes.

## Key Parameters

- Context map between bounded contexts.
- Aggregate granularity.

## When To Use

Complex business domains where the model itself drives application structure; especially valuable when adopting microservices.

## Risks & Pitfalls

- Over-application leads to anemic or anaemic abstractions.
- Misaligned context boundaries cause excessive cross-context calls.

## Related Concepts

- [[concepts/microservices]]
- [[concepts/event-driven-architecture]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
