---
title: "Python List"
type: concept
tags: [python, dataframe, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt"]
confidence: high
---

## Definition

A Python list is an ordered, mutable sequence written with square brackets `[]`. Lists allow mixed types, duplicates, and integer-indexed access, and are foundational for assembling dataframe inputs and modeling time series.

## How It Works

Lists are dynamic arrays internally. Elements are accessed and mutated by integer position (`my_list[0]`, `my_list[2] = "x"`). Lists support methods like `append`, `insert`, `remove`, `sort`, `reverse`, and length via `len()`. Copying requires `B = A.copy()` rather than `B = A` (which creates a reference); shallow vs. deep copying behavior matters for nested lists.

## Key Parameters

- Element type heterogeneity (mixed types allowed)
- Mutability
- Order preservation (insertion order is kept)
- Slice and index access (`[i:j]`)
- Comprehensions for elegant construction

## When To Use

- Storing ordered, possibly mutable, sequences
- Building dataframe columns or model time series
- Iterating with a stable order
- Holding the result of dictionary `keys()` / `values()` after wrapping in `list()`

## Risks & Pitfalls

- Confusing `B = A` (reference) with `B = A.copy()` (real copy)
- Deep vs. shallow copy nuances for nested lists
- Inefficient membership tests (`in`) on large lists vs. sets

## Related Concepts

- [[concepts/python]]
- [[concepts/dictionary]]
- [[concepts/series]]
- [[concepts/dataframe]]

## Sources

- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
