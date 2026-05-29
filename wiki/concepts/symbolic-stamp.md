---
title: Symbolic Stamp
type: claim
id: concepts/symbolic-stamp
tags:
- hierarchical
- symbolic
- analog
- advanced
- multi-port
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/12-8-hierarchical-analysis-methods.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A symbolic stamp is a multi-port admittance (or transfer) matrix whose entries are symbolic expressions (typically stored as DDD or GPDD subgraphs), used to characterize a subcircuit once and then reuse it as a single element in a higher-level network analysis.

## How It Works

Given a partitioned MNA `[[A11, A12], [A21, A22]] x = [b1, b2]`, the Schur complement `Y2 = A21 A11^{-1} A12` is a multi-port admittance whose entries can be computed symbolically with DDD/GPDD. Each `Y2[i,j]` is a sub-DDD/GPDD that can be evaluated at any parameter sample. At the next hierarchy level, `Y2` is stamped like an n-port admittance element into the parent MNA. Sharing across nested instances saves work.

## Key Parameters

- Choice of port nodes (defines `A12` and `A21`).
- Underlying engine (DDD vs. GPDD) — affects cancellation behavior.
- Caching strategy for identical sub-circuit instances.

## When To Use

- Large analog ICs with repeated subcircuit instances (e.g., differential pairs, biasing chains).
- Hierarchical Monte Carlo where the same subcircuit must be evaluated many times.

## Risks & Pitfalls

- DDD-based stamps reintroduce cancellation; GPDD-based stamps stay cancellation-free.
- Many ports leads to large multi-rooted stamps; partition granularity matters.

## Related Concepts

- [[concepts/hierarchical-symbolic-analysis]]
- [[concepts/schur-decomposition]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
