---
title: "Shannon Expansion"
type: concept
tags: [foundational, bdd, logic, recursion]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt"]
confidence: high
---

## Definition

Shannon expansion writes a Boolean (or multilinear arithmetic) function as `f(x) = x_i * f|_{x_i=1} + bar(x_i) * f|_{x_i=0}`, decomposing it around a single variable into two cofactor functions. The arithmetic analog for determinants is `det(A) = a_{i,j} (-1)^{i+j} Minor(A,a_{i,j}) + Rem(A,a_{i,j})`.

## How It Works

Repeated application against a fixed variable order yields a binary expansion tree; sharing equal cofactor subtrees turns the tree into a DAG (the BDD). Each leaf is the constant function (0 or 1 for logic; numeric value or zero for algebraic).

## Key Parameters

- Variable order chosen for the expansion.
- Termination criterion (constant cofactor, singular minor, disconnected subgraph, etc.).

## When To Use

- Foundation for BDD/ROBDD construction.
- Foundation for DDD-style determinant expansion (using Laplace-cofactor as the binary decision).
- Foundation for GPDD-style spanning-tree enumeration (edge collapse/remove as the binary decision).

## Risks & Pitfalls

- Without sharing, the expansion tree is exponential.
- Bad variable order destroys the benefit of sharing.

## Related Concepts

- [[concepts/binary-decision-diagram]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/spanning-tree-enumeration]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
