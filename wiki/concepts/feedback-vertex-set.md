---
title: Feedback Vertex Set
type: claim
id: claim-feedback-vertex-set
tags:
- graph
- algorithm
- well-established
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

A feedback vertex set (FVS) is a set S ⊆ V(G) such that G - S has no cycles (i.e., is a forest). The graph may have multiple edges and loops. The minimum FVS problem asks for the smallest such S.

## How It Works

FVS is NP-complete on general graphs. FPT algorithms based on bounded-search (Kloks-Xiao):

1. Reduce: delete vertices in no cycle; deduplicate parallel edges; replace degree-1 / degree-2 patterns; remove loops (charging k).
2. In a reduced graph every vertex has degree ≥ 3.
3. The first ⌈3k/2⌉ vertices in degree-decreasing order include at least one of any FVS of size ≤ k (Lemma 2.90).
4. Branch on these (1.5k)^k subtree explorations.

Current best: 2.7^k randomized (Li-Nederlof 2020), 3.46^k deterministic (Iwata-Kobayashi 2021).

## Key Parameters

- Solution size k.
- f(k) = (1.5k)^k in the simple bounded-search variant; better with refinements.

## When To Use

- Deadlock removal in dependency graphs.
- Image segmentation (planar FVS).
- Compiler optimization (loop nest elimination).

## Risks & Pitfalls

- Loops and multi-edges are explicitly allowed; reductions transform them carefully.
- "Feedback edge set" is the easier dual problem (find a spanning forest); FVS asks for a vertex hitting set.

## Related Concepts

- [[concepts/cycle]]
- [[concepts/fixed-parameter-tractability]]
- [[concepts/forest]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
