---
title: Tunable Consistency
type: claim
id: concepts/tunable-consistency
tags:
- databases
- consistency
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Tunable consistency lets the application specify, per read or write, how many replicas must participate in the operation. The N/W/R parameters (total replicas, writes required, reads required) trade off latency, availability, and consistency.

## How It Works

Common settings include W=N (immediate consistency, slow writes), W=1 (fast writes but inconsistency window), R=N (always read latest, slow reads), and the quorum balance (W + R > N) where read and write quorums overlap. Implemented in Cassandra, DynamoDB, Riak, and many others.

## Key Parameters

- N: total replicas.
- W: writes acknowledged before success.
- R: reads consulted before returning.

## When To Use

Whenever different operations in the same application have different freshness requirements (e.g., reads of a profile vs. reads of an account balance).

## Risks & Pitfalls

- Misconfigured quorums produce silent stale-read bugs.
- Sloppy quorums + hinted handoff weaken guarantees further.

## Related Concepts

- [[concepts/quorum]]
- [[concepts/eventual-consistency]]
- [[concepts/sloppy-quorum]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
