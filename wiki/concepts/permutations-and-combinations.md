---
title: Permutations and Combinations
type: claim
id: claim-permutations-and-combinations
tags:
- statistics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
confidence:
  base: 0.85
---

## Definition

Combinations count unordered selections of r items from n: `nCr = n!/(r!(n-r)!)`. Permutations count ordered arrangements: `nPr = n!/(n-r)!`. Combinations ignore order; permutations multiply by `r!` to account for all orderings of the same selection.

## How It Works

The factorial `x! = x*(x-1)*...*1` defines the multiplicity. For example, 5C3 = 10 ways to pick three ice-cream flavors from five; 5P3 = 60 ways once the order matters.

## Key Parameters

- Population size n
- Selection size r
- Whether order matters

## When To Use

- Counting sample-space sizes for probability
- Combinatorial enumeration
- Designing experimental groupings

## Risks & Pitfalls

- Confusing permutations with combinations
- Overflow when n is large (use logs or scipy.special.comb)
- Forgetting repeated elements (use multinomial coefficients)

## Related Concepts

- [[concepts/probability]]
- [[concepts/symbolic-mathematics]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
