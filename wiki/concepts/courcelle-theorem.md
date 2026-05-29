---
title: Courcelle's Theorem
type: claim
id: claim-courcelle-theorem
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

Courcelle's Theorem (1990): every problem expressible in MS2 (monadic second-order logic with edge-set quantification) can be solved in linear time on graphs of bounded treewidth, with the constant depending on the formula and the treewidth.

The MS1 variant: every problem expressible in MS1 (quantification over vertices and vertex sets) can be solved in O(n^3) on graphs of bounded rankwidth.

## How It Works

The algorithm processes a tree-decomposition (or rank-decomposition) bottom-up, maintaining for each bag a finite list of states that capture the truth of all sub-formulas restricted to the bag. A finite-automaton-like step combines child-states at each node.

Bodlaender's linear-time tree-decomposition algorithm for bounded treewidth (or Hliněný-Oum's cubic rankwidth algorithm) feeds the meta-theorem.

## Key Parameters

- Treewidth tw or rankwidth rw (the parameter).
- Formula size (governs the hidden constant).
- Number of quantifier alternations.

## When To Use

- Quick proof that a problem is in FPT on bounded-treewidth graphs.
- Algorithm sketching for problems like Hamiltonian cycle, independent set, dominating set, etc.

## Risks & Pitfalls

- Hidden constants can be tower-of-exponentials in formula size.
- Not every NP problem is MS2-expressible.
- The theorem is an existence proof; concrete algorithms are often more efficient.

## Related Concepts

- [[concepts/monadic-second-order-logic]]
- [[concepts/treewidth]]
- [[concepts/rankwidth]]
- [[concepts/tree-decomposition]]
- [[concepts/fixed-parameter-tractability]]
- [[concepts/bakers-method]]

## Sources

- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
