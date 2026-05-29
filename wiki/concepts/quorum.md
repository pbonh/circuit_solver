---
title: Quorum
type: claim
id: concepts/quorum
tags:
- distributed-systems
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

A quorum is a majority of replicas required to perform an operation: floor(N/2) + 1. Quorum reads and writes (W + R > N) guarantee that the read and write sets always overlap, so a read sees the latest committed write.

## How It Works

Writes succeed when W replicas acknowledge; reads consult R replicas and return the freshest value. Consensus algorithms like Raft and Paxos also require a quorum to commit log entries or elect leaders.

## Key Parameters

- N replicas.
- W writes for success.
- R reads for success.

## When To Use

Any replicated system that needs to balance availability and consistency. The default in many NoSQL and consensus-based systems.

## Risks & Pitfalls

- Failing to reach a quorum blocks progress.
- Even quorum is not sufficient against arbitrary network partitions; sloppy quorums weaken guarantees.

## Related Concepts

- [[concepts/sloppy-quorum]]
- [[concepts/consensus]]
- [[concepts/tunable-consistency]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
