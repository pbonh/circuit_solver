---
title: "s-Expanded DDD"
type: concept
tags: [ddd, symbolic, ac, advanced, frequency-domain]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/08-4-determinant-decision-diagrams.txt"]
confidence: high
---

## Definition

An s-expanded DDD is a multi-rooted Determinant Decision Diagram in which each root represents the coefficient DDD of a particular power of the complex frequency `s` in the polynomial expansion `det(A(s)) = a_n s^n + ... + a_0`. Combined with the original "complex" DDD it forms a complete s-domain symbolic representation of the circuit matrix determinant.

## How It Works

Each matrix entry's admittance is labeled by type — resistive (`g`), capacitive (`c*s`), or inductive (`1/(l*s)`) — and the construction performs a single DFS of the complex DDD invoking `CoeffMultiply`, `CoeffUnion`, and shift operations `P*s`, `P/s`. Two labeling schemes: scheme 1 lumps all admittances of the same type in an entry into one symbol; scheme 2 keeps each parameter distinct (more terms but better suited for parameter-level approximations).

## Key Parameters

- Labeling scheme (1 vs. 2).
- Variable ordering across all admittance parameters.
- Polynomial degree `n` and admittance count per entry `k` (affects size `O(k n |D|)`).

## When To Use

- Generating s-expanded polynomials for transfer functions.
- Pole-zero analysis, noise modeling, symbolic approximation per power of `s`.
- Bound analysis on coefficients under process variation.

## Risks & Pitfalls

- Numerator and denominator share parameters but the number of coefficient DDDs grows linearly with `n`.
- Symbolic cancellations from MNA must be removed during or after construction.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/symbolic-approximation]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations]]
