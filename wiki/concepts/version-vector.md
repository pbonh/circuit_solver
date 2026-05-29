---
title: Version Vector
type: claim
id: concepts/version-vector
tags:
- distributed-systems
- consistency
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A version vector is a per-object data structure that maintains a logical clock per replica. By comparing vectors, a system can determine whether one update happens-before another or whether they are concurrent (a conflict).

## How It Works

Each replica maintains its own counter for an object. On an update at replica R, R increments its counter and ships the new vector to peers. A receiving replica accepts the write only if every counter in the new vector is greater than or equal to its local copy; otherwise the update is treated as concurrent and the system stores both versions ("siblings" in Riak) for application merge.

## Key Parameters

- Vector size (one entry per replica).
- Garbage-collection policy for retired replica entries.

## When To Use

Leaderless databases where concurrent writes must be detected rather than lost (Riak, DynamoDB).

## Risks & Pitfalls

- Vectors grow with replica count.
- Application must implement merge logic for siblings.

## Related Concepts

- [[concepts/logical-clock]]
- [[concepts/last-writer-wins]]
- [[concepts/crdt]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
