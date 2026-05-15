---
title: "Delta-Based Accumulative Iterative Computation (DAIC)"
type: concept
tags: [graph, distributed-systems, asynchronous, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Definition

DAIC is a computation model, introduced by Maiter, in which each iteration updates vertex values by accumulating value changes (deltas) rather than recomputing values from scratch. Under conditions of ⊕-distributivity, asynchronous prioritized delta propagation converges to the same exact fixed point as synchronous Pregel.

## How It Works

Each vertex v maintains both a(v) (its value) and Δa(v) (its accumulated pending change). When a delta message m = g_{u,v}(Δa(u)) arrives from in-neighbor u, v sets Δa(v) ← Δa(v) ⊕ m. When v's own UDF runs, it (1) applies a(v) ← a(v) ⊕ Δa(v), (2) sends g_{v,w}(Δa(v)) to each out-neighbor w, and (3) resets Δa(v) to the identity element of ⊕. Two correctness conditions: the update function can be written as `a^{i+1}(v) = (⊕_{u in in(v)} g_{u,v}(a^i(u))) ⊕ c(v)`, and `g_{u,v}(x ⊕ y) = g_{u,v}(x) ⊕ g_{u,v}(y)` (distributivity).

## Key Parameters

- Choice of ⊕ operator and identity element.
- Edge function g_{u,v} (e.g., 0.85·x/d_out(u) for PageRank, identity for Hash-Min).
- Constant term c(v) (e.g., 0.15 for PageRank, vertex ID for Hash-Min).
- Prioritization rule (e.g., process top-1% of vertices by |Δa(v)|).
- Convergence criterion polled periodically by the master.

## When To Use

- Iterative graph algorithms with monotonic value changes where most vertices converge early (PageRank, Hash-Min, SSSP).
- Settings where GraphLab's approximate results are unacceptable but GraphLab-style fast convergence is desired.
- Block-centric implementations that propagate deltas inside a block before sending across.

## Risks & Pitfalls

- Not every Pregel algorithm satisfies the distributivity condition.
- Termination detection requires a global convergence sync, often via aggregator.
- Reference implementation (Maiter) is built on MapReduce, which limits performance; ideas may need re-implementation on a faster runtime.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/block-centric-computation]]
- [[concepts/shared-memory-graph-abstraction]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
