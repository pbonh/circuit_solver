---
title: Grundy Value
type: claim
id: claim-grundy-value
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

For a finite impartial game represented as a DAG of positions (arc x → y means y is reachable from x in one move), the Grundy value (Sprague-Grundy number) of a position is defined recursively:
- Every sink has Grundy value 0.
- For non-sink position x with out-neighbors y_1, …, y_k, Grundy(x) = mex{Grundy(y_i) : i ∈ [k]}, the minimum excluded non-negative integer.

The first player has a winning strategy from x iff Grundy(x) ≠ 0.

## How It Works

The sum theorem: Grundy(A + B) = Grundy(A) ⊕ Grundy(B) using nim-sum (XOR). This decomposes games into independent components.

Applications in the book:
- Snake game: player 1 wins iff the graph has no perfect matching.
- NIM: Grundy value = nim-sum of pile sizes.
- Chomp on bipartite graphs: Grundy = n_2 + 2·m_2 (mod 2).

## Key Parameters

- Grundy value ∈ ℕ ∪ {0}.
- Game termination required (no infinite play).

## When To Use

- Strategy computation for combinatorial games.
- Decomposing complex games into independent subgames.

## Risks & Pitfalls

- The DAG must be acyclic (finite games); cycles invalidate the recursion.
- The mex computation can be exponential in the position graph size; structured games (poset games, coin-turning) admit polynomial-time computations.

## Related Concepts

- [[concepts/nim-sum]]
- [[concepts/dag]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
