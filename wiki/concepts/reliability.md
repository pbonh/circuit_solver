---
title: Reliability
type: claim
id: claim-reliability
tags:
- distributed-systems
- foundational
- well-established
- fault-tolerance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

Reliability is a system's ability to "continue to work correctly, even when things go wrong." A reliable system performs the function the user expected, tolerates user mistakes and unexpected usage, performs adequately under expected load, and prevents unauthorized access. Faults (components deviating from spec) are distinguished from failures (the system as a whole no longer providing its service): fault-tolerance designs prevent faults from causing failures.

## How It Works

- Hardware faults are mitigated through redundancy (RAID, dual power supplies, replicated storage) and, increasingly, software-level fault tolerance across machines so that whole machines can fail.
- Software faults are systematic and correlated across nodes; mitigations include carefully checking assumptions, process isolation, allowing-and-restarting on crash, and continuous self-monitoring.
- Human-error mitigations: design APIs and interfaces that make the right thing easy; provide non-production sandboxes; layer unit, integration, and manual tests; allow quick rollback and gradual rollout; instrument with detailed telemetry.
- Deliberate fault injection (e.g., Netflix Chaos Monkey) keeps fault-handling code paths exercised so they are reliable when real faults occur.

## Key Parameters

- Mean time to failure / mean time to recovery of components.
- Tolerable fault classes (single-node crash, datacenter loss, etc.).
- Required availability target (e.g., 99.9% uptime).
- Acceptable degradation modes for partial failures.

## When To Use

Always — reliability is a baseline property required for any system whose user impact, financial cost, or legal exposure makes outages unacceptable, which today is essentially all production software, not just nuclear plants or air traffic control.

## Risks & Pitfalls

- Hardware redundancy alone is insufficient as fleets grow: hardware faults become routine.
- Systematic software faults are highly correlated and harder to anticipate than random hardware faults.
- Configuration errors by operators are a leading cause of outages; restrictive interfaces sometimes get worked around.
- Over-investing in tolerating very rare scenarios (e.g., p99.99) yields diminishing returns and may not be cost-effective.

## Related Concepts

- [[concepts/fault-tolerance]]
- [[concepts/scalability]]
- [[concepts/maintainability]]
- [[concepts/response-time-percentiles]]

## Sources

- [[summaries/ddia-02-preface]]
- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
