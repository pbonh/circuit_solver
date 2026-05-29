---
title: Sloppy Quorum
type: claim
id: concepts/sloppy-quorum
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

A sloppy quorum, first described in the Dynamo paper, accepts writes even when the canonical home replicas are unreachable. Substitute nodes temporarily store the writes and forward them (via hinted handoff) once the home replicas return.

## How It Works

When fewer than W canonical replicas are reachable, the coordinator picks an alternate live node to hold the write. After partition healing, the alternate forwards the value to the original home node. Implemented in DynamoDB, Cassandra, Riak, and Voldemort.

## Key Parameters

- Substitute-node selection policy.
- Hint retention timeout.

## When To Use

When write availability is more important than strict freshness guarantees.

## Risks & Pitfalls

- Even with R + W > N, reads can still miss the latest write while the hint is pending.
- Hint retention queues can balloon during long partitions.

## Related Concepts

- [[concepts/quorum]]
- [[concepts/hinted-handoff]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
