---
title: False Cognate
type: claim
id: claim-false-cognate
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

A false cognate is a term that looks or sounds identical across two bounded contexts but carries a different meaning in each. Conflating the two meanings causes integration bugs because developers assume a single shared definition.

## How It Works

False cognates surface at context boundaries when the same word is used in both contexts. Without an explicit translation table in the context map, code passing the term from one context to another silently changes its semantics.

## Key Parameters

- The term string.
- Meaning in context A.
- Meaning in context B.
- The invariant or type that breaks when conflated.

## When To Use

During strategic design and context-map construction, and during code review at module boundaries.

## Risks & Pitfalls

- The most dangerous false cognates are terms that are almost the same — close enough to feel familiar, different enough to introduce subtle bugs.
- Organizational silos exacerbate the problem because each team assumes their definition is universal.

## Related Concepts

- [[concepts/bounded-context]]
- [[concepts/context-map]]
- [[concepts/domain-driven-design]]

## Sources

- (Stub — created during Strategy workflow for circuit-solver. Expand with raw source on DDD.)
