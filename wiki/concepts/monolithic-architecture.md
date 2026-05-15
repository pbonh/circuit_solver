---
title: "Monolithic Architecture"
type: concept
tags: [foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: high
---

## Definition

A monolithic architecture packages all application functionality — API endpoints, business logic, data access — into a single executable artifact deployed as one process. It is the historical default for enterprise applications.

## How It Works

A web framework (Spring, Rails, Django) routes requests to handler methods in the same process. All handlers share a database, a connection pool, and runtime memory. Scaling out replicates the entire monolith and load-balances across copies.

## Key Parameters

- Codebase size.
- Build/deploy frequency.

## When To Use

Early-stage products, small teams, applications with modest scale where the operational complexity of microservices is unjustified.

## Risks & Pitfalls

- Becomes "the big ball of mud" as features accumulate.
- Cannot scale subsystems independently — everything scales together.
- Single deployment unit means slow release cadence and large blast radius.

## Related Concepts

- [[concepts/microservices]]
- [[concepts/horizontal-scaling]]
- [[concepts/backend-for-frontend]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
