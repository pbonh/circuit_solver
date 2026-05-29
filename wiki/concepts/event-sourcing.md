---
title: Event Sourcing
type: claim
id: claim-event-sourcing
tags:
- well-established
- distributed-systems
- derived-data
- domain-driven-design
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.85
---

## Definition

Event sourcing is a pattern (popularized in the domain-driven design community) in which an application's state is stored as an append-only log of immutable events representing user actions, and the current state is derived by replaying or aggregating those events. It differs from change data capture in that events are modeled at the application's intent level ("course cancelled") rather than as low-level row mutations.

## How It Works

- Every state-changing user request is validated; if valid, an immutable event describing the action is appended to the event log.
- Derived state (read models, indexes, caches) is built by stream processors that consume the event log deterministically.
- Replaying the log reproduces state; periodic snapshots speed up recovery.
- Commands (which may fail validation) are distinct from events (which are facts once accepted).
- Pairs naturally with CQRS: writes go to the event log, reads come from derived views.

## Key Parameters

- Event granularity (one-event-per-user-action vs aggregate-of-multiple-changes).
- Snapshot cadence.
- Event schema versioning policy.

## When To Use

For systems with strong audit/regulatory requirements, complex business logic, evolving read models, multi-replica derived views, or where understanding historical state is valuable (accounting, healthcare, e-commerce order fulfillment).

## Risks & Pitfalls

- Log compaction is awkward — events typically express intent, so later events don't supersede earlier ones cleanly.
- Schema evolution of events must be backward-compatible forever.
- Eventual consistency between event log and read views requires explicit timeliness handling.
- Mistakes in events are corrected by appending compensating events, not editing history.

## Related Concepts

- [[concepts/cqrs]]
- [[concepts/change-data-capture]]
- [[concepts/log-based-message-broker]]
- [[concepts/derived-data]]
- [[concepts/idempotency]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
