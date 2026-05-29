---
title: Minor Hash (DDD Implementation)
type: claim
id: claim-minor-hash
tags:
- ddd
- bdd
- implementation
- hash
- sparse-matrix
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/09-5-ddd-implementation.txt
confidence:
  base: 0.85
---

## Definition

The Minor Hash is a content-addressable data structure used by Layered-Expansion-Diagram (LED) DDD construction. It maps a minor (sub-determinant) — uniquely identified by its sorted row-index set and column-index set under a fixed symbol order — to the segment head element previously created for that minor.

## How It Works

Theorem 5.1 of the chapter establishes that under a fixed symbol order, two minors sharing identical row/column index sets must be entry-wise identical. Thus the pair `(row_indices, col_indices)` is a complete canonical key. During LED construction, whenever a row/column is selected from the current minor, the reduced minor's index sets are looked up; if present, the existing segment is reused (sharing); else inserted. Row/column degrees are stored alongside for fast min-degree heuristic and singularity detection.

## Key Parameters

- Hash function over integer index sets.
- Row/column-degree caching for the min-degree heuristic.

## When To Use

- Replacing the triple-based hash in DDD packages dependent on a logic-BDD library.
- Allows sign determination on-the-fly (from `(r,c)` position) without a second pass.

## Risks & Pitfalls

- Requires that the minor index sets be normalized/sorted before hashing.
- Loses validity if expansion does not follow the symbol-order assumption.

## Related Concepts

- [[concepts/layered-expansion-diagram]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/binary-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation]]
