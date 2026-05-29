---
title: Array Slicing
type: claim
id: concepts/array-slicing
tags:
- python
- numpy
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Array slicing extracts a subset of an array using integer indices, slice notation `start:stop:step`, or Boolean masks. NumPy slicing supports multi-axis indexing such as `x[:, 1]` for a column or `x[x < 5]` for conditional selection.

## How It Works

Each axis accepts its own slice. Integers select a single position; `start:stop:step` selects a range; ellipsis fills remaining axes; Boolean arrays select matching elements. The book notes the slice yields a copy when conditions are used; basic slicing typically gives a view.

## Key Parameters

- Slice triple (start, stop, step)
- Boolean mask array
- Axis order in multi-dim indexing

## When To Use

- Selecting columns, rows, or sub-blocks
- Filtering rows by a predicate
- Building training/test splits

## Risks & Pitfalls

- Basic slices share memory — mutations propagate
- Mixing fancy and basic indexing produces surprising results
- Out-of-range integer indices raise `IndexError`

## Related Concepts

- [[concepts/ndarray]]
- [[concepts/array-reshaping]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
