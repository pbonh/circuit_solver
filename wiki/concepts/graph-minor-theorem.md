---
title: Graph Minor Theorem
type: claim
id: concepts/graph-minor-theorem
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Graph Minor Theorem (Robertson-Seymour, 1985-2004): the class of all finite simple graphs is well-quasi-ordered by the minor relation. Equivalently, every class of graphs closed under taking minors has a finite obstruction set.

## How It Works

H is a minor of G iff H can be obtained from G by a sequence of vertex deletions, edge deletions, and edge contractions. Equivalently, V(G) can be partitioned into "branch sets" V_1, …, V_h (where V(H) = [h]) such that each G[V_i] is connected and every edge of H is realized between some pair of branch sets.

Consequences:
- For any minor-closed class G, there are finitely many minimal forbidden minors (obstructions).
- The class of planar graphs has obstruction set {K_5, K_{3,3}} (Kuratowski-Wagner).
- Outerplanar graphs: obstructions {K_4, K_{2,3}}.
- Bounded treewidth: a finite (but large) obstruction set.

Algorithmically: for fixed H, testing "H minor of G?" is in O(n^3) (FPT in |H|).

## Key Parameters

- Width parameters (treewidth, branchwidth) often appear in minor-closed classes.
- Obstruction set is finite (existence is guaranteed; size may be huge).

## When To Use

- Proving the existence of polynomial-time algorithms for minor-closed problems via Courcelle / Bodlaender.
- Justifying that any "nice" graph property has a finite local characterization.

## Risks & Pitfalls

- The obstruction set is finite but typically unknown explicitly.
- Robertson-Seymour proof spans more than 20 papers and is one of the longest proofs in mathematics.

## Related Concepts

- [[concepts/minor]]
- [[concepts/well-quasi-order]]
- [[concepts/treewidth]]
- [[concepts/outerplanar-graph]]
- [[entities/robertson-seymour-graph-minors]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
