---
title: Reduced Ordered BDD (ROBDD)
type: claim
id: concepts/robdd
tags:
- bdd
- foundational
- canonical
- data-structure
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Reduced Ordered Binary Decision Diagram (ROBDD) is a BDD obeying two rules under a fixed total variable order: (i) any node whose two children are identical is removed; (ii) any two nodes with the same `(var, solid, dashed)` triple are merged. Under these rules the ROBDD is canonical — uniquely determined by the function it represents.

## How It Works

Build bottom-up using a unique-table hash on `(var, solid_ptr, dashed_ptr)`; the lookup returns the existing node or creates a new one. Operations (AND, OR, ITE) recurse on the top variable and memoize results. Bryant (1986) established the canonicity theorem and provided efficient algorithms.

## Key Parameters

- Variable order (NP-complete to optimize).
- Unique-table hash and memoization-cache sizes.

## When To Use

- Logic verification (functional equivalence by pointer comparison).
- Symbolic state-space exploration.
- Anywhere a canonical compact representation of Boolean (or analogous) functions is needed.

## Risks & Pitfalls

- Variable order dominates size; dynamic reordering (sift) helps but is expensive.
- Pathological functions (e.g., multipliers) have exponential ROBDDs regardless of order.

## Related Concepts

- [[concepts/binary-decision-diagram]]
- [[concepts/shannon-expansion]]
- [[concepts/variable-ordering]]
- [[concepts/ite-operator]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
