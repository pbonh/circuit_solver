---
title: "Bryant 1986 BDD Paper"
type: entity
tags: [paper, bdd, foundational, canonical, history]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt"]
confidence: medium
---

## Overview

Randal E. Bryant's 1986 IEEE Transactions on Computers paper "Graph-Based Algorithms for Boolean Function Manipulation" established the canonicity of the Reduced Ordered Binary Decision Diagram (ROBDD) under a fixed variable order, and gave systematic algorithms for Boolean operations. It is widely regarded as the founding work of modern BDD-based EDA.

## Characteristics

- Defined ROBDD via the two reduction rules (merging isomorphic subgraphs, removing don't-care nodes).
- Proved canonicity given a fixed variable order.
- Provided algorithms for AND, OR, NOT, composition, restriction, and equivalence checking.

## Common Strategies

- Cited as the foundational reference in essentially every BDD-based work.
- Variable-order sensitivity highlighted in this paper motivated decades of ordering-heuristic research.

## Related Entities

- [[entities/scad3]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
