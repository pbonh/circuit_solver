---
title: Context Map
type: claim
id: claim-context-map
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

A context map is a DDD pattern that documents the relationships, translation rules, and integration patterns between bounded contexts. It is the strategic counterpart to the tactical model inside each context, preventing language drift and false-cognate bugs at boundaries.

## How It Works

A context map is drawn as a graph whose nodes are bounded contexts and whose edges are labeled with integration patterns (shared-kernel, customer-supplier, conformist, anticorruption-layer, published-language, open-host-service, separate-ways). Each edge may carry a translation table for terms that cross the boundary.

## Key Parameters

- The set of bounded contexts participating.
- Integration pattern per context pair.
- Translation table for cross-boundary terms.
- False-cognate inventory.

## When To Use

After bounded contexts have been identified, before writing specs or drawing architecture diagrams. The context map is a living document that should be revisited when contexts are split, merged, or when new integration paths are added.

## Risks & Pitfalls

- Drawing the map once and never updating it — the map becomes fiction.
- Omitting the translation table — the most common source of boundary bugs.
- Picking integration patterns based on team politics rather than coupling reality.

## Related Concepts

- [[concepts/bounded-context]]
- [[concepts/false-cognate]]
- [[concepts/domain-driven-design]]

## Sources

- (Stub — created during Strategy workflow for circuit-solver. Expand with raw source on DDD.)
