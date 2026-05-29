---
title: Hyperscale
type: claim
id: claim-hyperscale
tags:
- scalability
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.65
---

## Definition

Hyperscale systems exhibit exponential growth in computational and storage capacity while keeping the costs of building, operating, and evolving the system on a linear trajectory. The major internet companies are the canonical examples.

## How It Works

Achieved via aggressive horizontal scale-out on commodity hardware, software-defined infrastructure, automation/DevOps, custom hardware where economical, and architectural patterns that avoid serial bottlenecks. Internal platforms (storage, compute, messaging) are productized for reuse across many services.

## Key Parameters

- Ratio of cost growth to traffic growth.
- Self-service infrastructure availability.

## When To Use

Mostly aspirational for individual products; the principles inform any design that must scale across multiple orders of magnitude.

## Risks & Pitfalls

- The economics rarely apply to smaller systems.
- Demanding patterns may be premature.

## Related Concepts

- [[concepts/scalability]]
- [[concepts/horizontal-scaling]]
- [[concepts/devops]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
