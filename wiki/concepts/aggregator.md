---
title: Aggregator (Pregel)
type: claim
id: concepts/aggregator
tags:
- graph
- distributed-systems
- pregel
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

A Pregel aggregator is a global reduction primitive: every vertex contributes a value during a superstep, the system reduces these values, and the aggregated result is broadcast back to all vertices for use in the next superstep.

## How It Works

Each machine maintains a partial aggregator state. During a superstep, vertices supply values via the aggregator API; the machine performs local reduction. At the superstep boundary, partial results are gathered to the master, globally reduced, and the result is broadcast to every machine so that any vertex can read it in the next superstep. Common uses include termination detection (e.g., counting non-converged vertices), tracking the smallest/largest vertex ID seen globally, or sharing the current best known objective in branch-and-bound mining (used analogously by G-thinker's aggregator with periodic global synchronization).

## Key Parameters

- Reduction operator (sum, min, max, or user-defined associative-commutative function).
- Synchronization frequency (every superstep in Pregel; configurable in G-thinker).

## When To Use

- Detecting global convergence (e.g., no vertex updated in a round in S-V).
- Sharing a global statistic (current best clique size, number of active queries).
- Coordinating non-trivial termination conditions across many machines.

## Risks & Pitfalls

- Synchronous global aggregation is a serializing point that can become a bottleneck.
- Stale aggregator values in asynchronous variants can hurt pruning effectiveness if periodic sync is too coarse.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/message-combiner]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
