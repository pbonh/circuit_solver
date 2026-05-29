---
title: Message-Logging Recovery
type: claim
id: concepts/message-logging-recovery
tags:
- distributed-systems
- fault-tolerance
- graph-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Message-logging recovery is a Pregel fault-tolerance technique in which each worker persists the messages it generates to local disk before sending them, so that on failure only the messages destined for failed (reloaded) vertices need to be replayed by surviving workers, avoiding a full coordinated rollback.

## How It Works

During normal execution each worker writes a local log of outgoing messages — cheap on commodity clusters because Gigabit-Ethernet bandwidth is the bottleneck and disk streaming runs concurrently with transmission. On failure at superstep s_fail with the latest checkpoint at s_cp: surviving vertices remain at s_fail (their state is intact), while failed-machine vertices reload from s_cp. Instead of all workers re-executing s_cp..s_fail, surviving workers replay their logged messages targeting the reloaded vertices. A robust algorithm handles cascading failures. A refinement (Yan et al.) logs only vertex states rather than full messages, regenerating messages from the logged states during recovery to reduce log growth.

## Key Parameters

- Log compaction frequency (must align with new checkpoints to bound disk usage).
- Whether to log raw messages or vertex states.
- Underlying network/disk bandwidth ratio.

## When To Use

- Long-running jobs on clusters where partial recovery is critical.
- Environments where local-disk streaming bandwidth exceeds network bandwidth (commodity Gigabit Ethernet).

## Risks & Pitfalls

- Failure-free execution is slowed by log writes and by periodic log truncation aligned with checkpoints.
- Recovery algorithm must be carefully written to be robust against cascading failures.
- Asynchronous models (GraphLab) need uncoordinated-snapshot variants (Chandy-Lamport).

## Related Concepts

- [[concepts/lightweight-checkpointing]]
- [[concepts/fault-tolerance]]
- [[concepts/bulk-synchronous-parallel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
