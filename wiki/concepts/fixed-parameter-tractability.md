---
title: Fixed-Parameter Tractability (FPT)
type: claim
id: concepts/fixed-parameter-tractability
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A parameterized problem is a pair (P, k) where k differentiates input instances. The problem is fixed-parameter tractable (FPT) if there is an algorithm with runtime O(f(k) · |P|^c) for a computable function f and constant c independent of k.

The class W[0] = FPT; problems are W[1]-hard if no FPT algorithm exists under standard complexity assumptions (Downey-Fellows W-hierarchy).

## How It Works

The Bounded Search technique is a recurring tool: build a search tree of depth k where each branch makes "progress" toward the parameter bound. Examples:
- Vertex cover: 2^k branches (pick endpoint of an uncovered edge) — O(2^k · n).
- Edge dominating set: 4^k — by enumerating minimal vertex covers of size 2k.
- Feedback vertex set: (1.5k)^k via high-degree branching.

Kernelization, treewidth-based DP, and color-coding are other FPT techniques.

## Key Parameters

- The parameter k (often solution size).
- f(k) characterizes the exponential blow-up; for many problems f is single-exponential 2^O(k).

## When To Use

- When the natural parameter (solution size, treewidth, vertex cover number) is small in practice.
- For problems like FVS, Steiner tree, vertex cover, where polynomial-time exact solutions don't exist.

## Risks & Pitfalls

- W[1]-hard problems (e.g. clique parameterized by clique size) admit no known FPT algorithm.
- The constant c in n^c is fixed, but f(k) can be tower-of-exponentials.

## Related Concepts

- [[concepts/vertex-cover]]
- [[concepts/edge-dominating-set]]
- [[concepts/feedback-vertex-set]]
- [[concepts/treewidth]]
- [[concepts/np-completeness]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
