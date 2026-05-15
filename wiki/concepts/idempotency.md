---
title: "Idempotency"
type: concept
tags: [distributed-systems, fault-tolerance, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

An operation is idempotent if applying it multiple times produces the same result as applying it once. In distributed systems, idempotent APIs make safe retry possible in the face of unknown delivery outcomes after partial failure.

## How It Works

The client attaches a unique idempotency key (session id + timestamp/UUID/sequence number) to each mutating request. The server stores the key alongside the application state change in a single transaction; if a duplicate request arrives, the server detects the key, skips the mutation, and returns a stored response. Keys are pruned after a TTL (typically 60 min - 24 h).

## Key Parameters

- Idempotency-key format and uniqueness guarantee.
- Server-side key TTL.
- Transactional coupling of state change and key storage.

## When To Use

Every mutating API in a distributed system that can be retried — payments, order placement, account updates, etc.

## Risks & Pitfalls

- Storing the key without storing the state change (or vice versa) silently corrupts behavior.
- Key collisions across clients can suppress legitimate distinct requests.

## Related Concepts

- [[concepts/exactly-once-processing]]
- [[concepts/partial-failure]]
- [[concepts/poison-message]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
