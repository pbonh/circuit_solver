---
title: 'Advanced Symbolic Analysis for VLSI Systems — Chapter 5: DDD Implementation'
type: source
id: summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation
kind: publication
tags:
- ddd
- bdd
- implementation
- complexity
- advanced
- sparse-matrix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/09-5-ddd-implementation.txt
---

## Key Points

- DDD performance is dominated by two interacting factors: variable order (sets final DDD size) and hash design (sets construction speed). A 2x2 grid (good/bad) for hash and order helps frame fair comparisons.
- Early DDD implementations used third-party BDD packages with triple-based hashing (vertex `(top, solid, dashed)`); these required a separate sign-determination pass and suffered from minor re-expansion on cache misses.
- Theorem 5.1 (key result of the chapter): under a fixed symbol order, any minor is uniquely identified by its row and column indices alone — no entry comparison is needed.
- The Layered Expansion Diagram (LED) algorithm builds a per-layer queue of segments. Each segment shares one minor; expanding any element produces a smaller minor that is enqueued in the next layer. Sharing is enforced via a Minor Hash Table keyed on `(row_set, col_set)`.
- LED advantages: no a-priori variable order, no third-party BDD package, no separate sign-determination pass.
- LED paths may not be "well-ordered" (different paths can traverse variables in different orders); strict canonicity is sacrificed but symbolic correctness is preserved. The chapter argues canonicity is less important for analog symbolic analysis since BDDs are not used here to verify function equality.
- Property: the in-segment expansion order is immaterial for DDD size, but the inter-layer row/column choice (min-degree heuristic recommended) matters greatly.
- An augmented-MNA formulation puts the input/output transfer function `H` itself as a symbol via `det([[A, b], [e_v^T - H e_u^T, 0]]) = 0`, yielding `H = N/D` from a single DDD expansion — avoiding two separate expansions for numerator and denominator.
- Experimental results: LED beats Greedy-Labeling on full matrices by >30x in DDD size at n=18 (LED: 2.36e6 nodes; Greedy: 6.21e7 nodes). On the muA725 op-amp, LED's faster construction wins even though its DDD is larger.
- Optimality result for full n x n matrices: rowwise (or columnwise) order yields the minimum DDD size `|DDD| = n * 2^{n-1}` (Theorem 5.4); growth rate is approximately 2 vs. the explicit-expansion factor of ~n in `n!`. Brute-force binary decomposition without sharing needs `O(n!)` nodes.
- For sparse matrices, neither rowwise nor columnwise is necessarily optimal; the optimal order is matrix-dependent and finding it remains an open problem. Greedy-Labeling is a useful heuristic in practice but not optimal even for the small 4x4 sparse example given.

## Relevant Concepts

- [[concepts/layered-expansion-diagram]] — chapter's main algorithm.
- [[concepts/determinant-decision-diagram]] — DDD properties developed further here.
- [[concepts/minor-hash]] — the chapter's key data-structure technique (row/column indices as canonical minor identifier).
- [[concepts/variable-ordering]] — DDD-specific implications for full and sparse matrices.
- [[concepts/modified-nodal-analysis]] — input formulation.
- [[concepts/symbolic-analysis]]
- [[concepts/binary-decision-diagram]] — substrate; differences from logic-BDD implementation strategies discussed.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 5 — DDD Implementation
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/09-5-ddd-implementation.txt`
- Author: Guoyong Shi
