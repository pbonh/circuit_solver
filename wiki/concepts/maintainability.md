---
title: "Maintainability"
type: concept
tags: [distributed-systems, foundational, well-established, software-engineering]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

Maintainability is the property that the people who later work on a software system — operations and engineering, current and future — can do so productively. Since the majority of software cost is in ongoing maintenance, not initial development, maintainability is decomposed into three design principles: operability, simplicity, and evolvability.

## How It Works

- **Operability**: make routine operational tasks easy. Provide visibility/telemetry, automation hooks, predictable behavior, sensible defaults with overrides, self-healing where appropriate, and freedom from single-machine dependencies so nodes can be patched without downtime.
- **Simplicity**: manage complexity by removing accidental complexity (Moseley & Marks). Good abstractions hide implementation details behind clean interfaces and become reusable. High-level languages, SQL, etc. are examples; in distributed systems, good abstractions are still scarce.
- **Evolvability**: make change easy. Agile/TDD/refactoring help at the code-file scale; at the data-system scale, evolvability is closely tied to simplicity and depends on schema-evolution discipline, rolling upgrades, and the ability to refactor architectures (e.g., Twitter timeline approach 1 → approach 2).

## Key Parameters

- Operational metrics: MTTR, change-failure rate, deployment frequency.
- Code-quality metrics: cyclomatic complexity, dependency depth, test coverage.
- Migration overhead: time/cost to evolve schemas, rewrite data, switch storage engines.

## When To Use

Always, on any long-lived system. Maintainability decisions made early compound over the lifetime of the system; the cost of legacy debt usually dwarfs the cost of writing it correctly the first time.

## Risks & Pitfalls

- Adding features without removing accidental complexity creates a "big ball of mud."
- Bad abstractions are worse than no abstractions — they leak, and they are very expensive to replace.
- Operability is sometimes neglected because automation is invisible until it fails.
- "Move fast and break things" without rollback tooling forces operators to absorb avoidable risk.

## Related Concepts

- [[concepts/reliability]]
- [[concepts/scalability]]
- [[concepts/schema-evolution]]
- [[concepts/backward-and-forward-compatibility]]

## Sources

- [[summaries/ddia-00-copyright]]
- [[summaries/ddia-02-preface]]
- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
