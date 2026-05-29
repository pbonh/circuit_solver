---
title: Pointer Jumping (Path Doubling)
type: claim
id: claim-pointer-jumping
tags:
- graph
- parallel
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
confidence:
  base: 0.85
---

## Definition

Pointer jumping (also called path doubling) is a parallel algorithm technique where each element in a chain or tree replaces its pointer with its pointer's pointer, halving the distance to the root in each round and giving O(log n) iterations to reach the root from any node.

## How It Works

Given a structure with parent pointers `pred(v)`, in each round every active vertex v reads `pred(v)` and `pred(pred(v))` and updates `pred(v) ← pred(pred(v))`. After O(log n) rounds every node points directly to the root (or to null). Distances or accumulated values along the chain (e.g., list ranks, sums) can be propagated in tandem.

## Key Parameters

- Number of rounds, bounded by O(log n).
- Termination: when all `pred(v) = null` (or roots have self-loops).
- Whether each round is implemented as a constant-bounded number of Pregel supersteps via request-respond.

## When To Use

- Parallel list ranking (computing the rank of each node in a linked list).
- Connected-components algorithms (Shiloach-Vishkin, S-V Pregel adaptation).
- Tree contraction, Euler-tour technique, biconnected components.
- Any time a Pregel algorithm needs an O(log |V|) bound rather than O(diameter).

## Risks & Pitfalls

- Communication is no longer limited to graph neighbors — vertices must talk to arbitrary other vertices by ID, which limits some Pregel optimizations.
- A naive implementation in Pregel requires multiple supersteps per round (request, respond, update); a request-respond API (Pregel+) reduces overhead.
- Memory at parent vertices can blow up if many descendants request state at once; combining requests by machine helps.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/bulk-synchronous-parallel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
