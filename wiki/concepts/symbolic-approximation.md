---
title: Symbolic Approximation (SBG/SDG/SAG)
type: claim
id: concepts/symbolic-approximation
tags:
- symbolic
- approximation
- analog
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Symbolic approximation discards insignificant product terms in a generated symbolic expression based on nominal numerical values and the frequency range of interest. It comes in three flavors keyed to when the discard happens: Simplification Before Generation (SBG), Simplification During Generation (SDG), and Simplification After Generation (SAG).

## How It Works

- SBG removes circuit elements whose contributions are negligible before term generation, reducing the input network.
- SDG generates terms in a non-increasing weight order (smallest-weight spanning trees, matroid-intersection enumeration, or BDD shortest-path), stopping when the next term is below threshold.
- SAG expands all terms first, then prunes by magnitude — the most reliable but most expensive.

## Key Parameters

- Approximation threshold (relative or absolute).
- Nominal parameter values and frequency range used to weight terms.
- Number of terms retained.

## When To Use

- SBG for very large circuits where full expansion is infeasible.
- SDG when ordered enumeration is available (tree enumeration, BDD shortest path).
- SAG only on small circuits where exact expansion fits in memory.

## Risks & Pitfalls

- Approximation accuracy depends on nominal values; design points far from nominal may need re-approximation.
- Frequency-range sensitivity: a term negligible at one frequency may dominate at another.

## Related Concepts

- [[concepts/symbolic-analysis]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/two-graph-method]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
