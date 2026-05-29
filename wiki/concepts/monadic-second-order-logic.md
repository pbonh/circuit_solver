---
title: Monadic Second-Order Logic
type: claim
id: claim-monadic-second-order-logic
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
confidence:
  base: 0.85
---

## Definition

Monadic second-order logic (MSO) is a logical language for expressing properties of structures (here graphs) where quantification ranges over both elements (vertices) and unary relations (vertex subsets). "Monadic" restricts the second-order quantifiers to sets, not arbitrary relations.

A sentence consists of logical symbols (parentheses, connectives ⇒, ∧, ∨, ¬), variables, quantifiers ∀, ∃, equality =, constants, and predicate symbols.

MS1 quantifies over vertices and vertex subsets. MS2 additionally allows quantification over edges and edge subsets.

## How It Works

Connectedness is expressed in MS1 by ∀A, B ⊂ V: (partition condition) ⇒ ∃a ∈ A, b ∈ B: {a,b} ∈ E. Hamiltonian cycle is expressible in MS2 (quantify over a set of edges forming a cycle that covers all vertices).

Courcelle's theorems:
- MS1 sentences are evaluable in O(n^3) time on graphs of bounded rankwidth.
- MS2 sentences are evaluable in linear time on graphs of bounded treewidth.

These are FPT meta-theorems: the algorithm is parameterized by treewidth/rankwidth and the formula size.

## Key Parameters

- Formula size.
- Treewidth (for MS2) or rankwidth (for MS1).
- Number of quantifier alternations bounds the constant in O(n^c).

## When To Use

- To prove that a problem is in FPT on bounded-treewidth or bounded-rankwidth graphs without designing an algorithm.
- For automatic algorithm synthesis from logical specifications.

## Risks & Pitfalls

- The hidden constants in Courcelle's theorems can be enormous (tower of exponentials in formula size).
- MS2 is strictly more expressive than MS1; not all MS2 problems are solvable in MS1 efficiency on the same graph class.

## Related Concepts

- [[concepts/courcelle-theorem]]
- [[concepts/treewidth]]
- [[concepts/rankwidth]]
- [[concepts/graph-algebra]]

## Sources

- [[summaries/guide-to-graph-algorithms-01-preface]]
- [[summaries/guide-to-graph-algorithms-06-problem-formulations]]
