---
title: 'Advanced Symbolic Analysis for VLSI Systems — Chapter 4: Determinant Decision
  Diagrams'
type: source
id: summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams
kind: publication
tags:
- ddd
- bdd
- symbolic
- analog
- advanced
- ac
- sparse-matrix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/08-4-determinant-decision-diagrams.txt
---

## Key Points

- A DDD is a signed, rooted, directed acyclic graph (essentially a ZBDD) whose 1-paths from root to the 1-terminal correspond bijectively to product terms in the full determinant expansion of an MNA-like sparse matrix.
- Each non-terminal vertex `a_i` carries a sign `s(a_i)` computed from row/column indices via the "sign rule" `s(v) = prod_{x in P(v)} sign(r(x)-r(v)) * sign(c(x)-c(v))`; the product of signs along a path gives the sign of the corresponding product term.
- Under a fixed variable ordering, a DDD is canonical (unique) and applies the ZBDD reduction rules: zero-suppression, ordering, and sharing.
- DDD compression can be extreme: in the cited example, 5.71x10^20 product terms fit in 398 vertices.
- Basic DDD operations parallel ZBDD operations: VertexOne/Zero, Cofactor (derivative wrt parameter), Remainder, Multiply (Change), Subtract (Diff), Union, Evaluate; all run in time linear in DDD size when memoized.
- Logic-operation DDD construction: build a Boolean function `f_det = f_row AND f_col` that "detects" valid product terms (each row and column hit exactly once), construct it as a BDD, then convert to ZBDD/DDD; row/column AND-of-OR construction makes BDD generation cheap.
- s-expanded DDDs: each admittance parameter (`g`, `c*s`, or `1/(l*s)`) is labeled separately so the determinant `det(A(s)) = sum_i a_i s^i` becomes a multi-rooted DDD with one coefficient-DDD per power of `s`. Two labeling schemes: (1) lump same-type admittances per entry, (2) one symbol per parameter.
- s-expanded DDD construction is a single DFS pass with `CoeffMultiply`, `CoeffUnion`, `P*s` (shift), `P/s` (shift). Complexity is `O(k n |D_r|)` where `k` is the max admittance count per entry.
- Symbolic cancellation: MNA formulation produces 70-90% cancellable terms because MNA is a reduction of the cancellation-free sparse tableau; cancellation can be removed during or after s-expanded DDD construction.
- Dominant-term extraction by incremental k-shortest-path on the reverse DDD: edge weights are `0` (zero-edge) and `-log|a_i|` (one-edge); the largest product term is the minimum-weight path. After subtracting a path, only newly-created vertices need relaxation, giving `O(|DDD| + n(k-1))` for the top-k dominant terms (`n` = DDD depth).

## Relevant Concepts

- [[concepts/determinant-decision-diagram]] — central topic; constructive definition, sign rule, canonicity.
- [[concepts/zero-suppressed-bdd]] — DDD is a signed ZBDD.
- [[concepts/binary-decision-diagram]] — substrate for the logic-operation construction.
- [[concepts/s-expanded-ddd]] — multi-rooted DDD with one coefficient DDD per power of `s`.
- [[concepts/symbolic-approximation]] — k-shortest-path on DDD for dominant-term extraction.
- [[concepts/modified-nodal-analysis]] — matrix formulation behind DDD.
- [[concepts/cramers-rule]] — basis for using cofactors and minors as transfer-function builders.
- [[concepts/symbolic-cancellation]] — MNA-induced cancellation, why DDDs need de-cancellation.
- [[concepts/variable-ordering]] — DDD size and complexity dominated by ordering.
- [[concepts/k-shortest-path]] — algorithmic primitive for dominant-term enumeration.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 4 — Determinant Decision Diagrams
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/08-4-determinant-decision-diagrams.txt`
- Author: Sheldon X.-D. Tan
