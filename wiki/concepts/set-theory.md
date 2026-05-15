---
title: "Set Theory"
type: concept
tags: [statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/07-chapter-3-regular-expressions-and-math-with-python.txt"]
confidence: low
---

## Definition

Set theory studies collections of distinct elements and operations on them — union, intersection, difference, and complement — typically visualized with Venn diagrams. The chapter uses SymPy's `FiniteSet` to compute set operations and basic probabilities.

## How It Works

A `FiniteSet` holds unique elements. The `union` method merges all distinct elements of two sets; `intersect` returns elements common to both. Probability of an event under equally likely outcomes is `|event| / |sample_space|`.

## Key Parameters

- Set membership predicate
- Cardinality (|S|)
- Operation choice (union, intersection, difference)

## When To Use

- Defining sample spaces in probability problems
- Reasoning about disjoint vs. overlapping categories
- Manipulating finite collections symbolically

## Risks & Pitfalls

- Infinite sets require special handling
- Mixing element types may yield unintuitive equality

## Related Concepts

- [[concepts/probability]]
- [[concepts/symbolic-mathematics]]

## Sources

- [[summaries/python-data-analysts-toolkit-07-chapter-3-regular-expressions-and-math-with-python]]
