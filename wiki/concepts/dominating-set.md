---
title: "Dominating Set"
type: concept
tags: [graph, foundational, well-established, np-hard]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A dominating set in a graph G is a set D ⊆ V such that every vertex of V \ D has a neighbor in D (equivalently, N[D] = V). The domination number γ(G) is the minimum cardinality of a dominating set.

A total dominating set additionally requires every vertex of V (including those in D) to have a neighbor in D; this requires no isolated vertices in G.

## How It Works

Computing γ is NP-complete in general (and on chordal graphs). Variants include bottleneck domination (minimize max weight), (a, b)-domination, paired domination, independence domination γ_i. AT-free graphs admit polynomial-time paired-domination algorithms.

Vizing's conjecture states γ(G □ H) ≥ γ(G) · γ(H); it is open in general but proved for chordal G by Aharoni-Szabó.

## Key Parameters

- γ(G).
- γ_i(G) = max over independent sets A of γ(A) (independence domination).
- "Domination number" of digraphs is defined analogously.

## When To Use

- Wireless / sensor network coverage.
- Facility location.
- Combined with matching to get "paired dominating sets" for redundancy.

## Risks & Pitfalls

- γ_i ≤ γ always, but they can differ widely.
- Edge-dominating set is different: it asks for a set of edges that dominate all other edges (equivalently, an independent dominating set in L(G)).

## Related Concepts

- [[concepts/graph]]
- [[concepts/bottleneck-domination]]
- [[concepts/edge-dominating-set]]
- [[concepts/vizings-conjecture]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
