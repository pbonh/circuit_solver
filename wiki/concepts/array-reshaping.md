---
title: Array Reshaping
type: claim
id: claim-array-reshaping
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
  base: 0.85
---

## Definition

Array reshaping changes the dimensionality of a NumPy array without changing its data. The `reshape` method returns a view with new dimensions; the `shape` attribute mutates the array in place; `ravel` returns a flattened 1-D view.

## How It Works

The product of the new shape must equal the total element count. Reshaping does not copy data — it reinterprets the memory layout with new strides. Multi-dimensional reshaping respects C-order by default.

## Key Parameters

- New shape tuple
- View vs. copy semantics
- `-1` as a placeholder for "infer this dimension"

## When To Use

- Adapting 1-D data for 2-D operations
- Preparing batches for neural networks
- Reverting to a flat layout via `ravel`

## Risks & Pitfalls

- Shape product mismatch raises a `ValueError`
- Views share memory — mutations affect the original
- Confusing row-major and column-major ordering

## Related Concepts

- [[concepts/ndarray]]
- [[concepts/array-slicing]]
- [[concepts/broadcasting]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
