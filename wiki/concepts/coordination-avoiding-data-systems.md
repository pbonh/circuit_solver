---
title: Coordination-Avoiding Data Systems
type: claim
id: concepts/coordination-avoiding-data-systems
tags:
- distributed-systems
- emerging
- performance
- consistency
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Coordination-avoiding data systems, proposed by Bailis et al. (2014), are systems that preserve application-defined integrity invariants without requiring synchronous coordination (locking, distributed transactions, leader election) for each operation. The insight: many real-world business invariants can be expressed in coordination-free ways or tolerate brief violation with compensating actions.

## How It Works

- Identify invariants the application truly requires (uniqueness, balance non-negativity, capacity limits).
- Determine which can be enforced **invariant confluence** style — i.e., concurrent operations preserve the invariant locally.
- For invariants that don't I-confluent, decide whether **temporary violation with compensating transactions** (apology, refund, overbooking adjustment) is acceptable.
- Use distributed dataflow with end-to-end IDs, idempotent updates, and asynchronous derivation, replacing 2PC with log-based integration.
- Synchronous coordination remains available for the small subset of operations that truly need it.

## Key Parameters

- Set of invariants and their I-confluence properties.
- Compensation/apology cost.
- Acceptable timeliness lag.

## When To Use

For geographically distributed systems where synchronous coordination is too slow, for high-throughput workloads where 2PC is impractical, and for systems where business processes already accommodate exceptions (banking, e-commerce, scheduling).

## Risks & Pitfalls

- Determining I-confluence is non-trivial and easy to get wrong.
- Compensation logic must be reliable and timely or invariants are violated in practice.
- Application code becomes more complex than a single ACID transaction.

## Related Concepts

- [[concepts/causal-consistency]]
- [[concepts/exactly-once-semantics]]
- [[concepts/end-to-end-argument]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
