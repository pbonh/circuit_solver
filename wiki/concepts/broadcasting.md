---
title: Broadcasting (NumPy)
type: claim
id: concepts/broadcasting
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

Broadcasting is NumPy's rule set for performing arithmetic between arrays of compatible but unequal shapes. The smaller array is implicitly stretched along missing or size-1 dimensions to match the larger array's shape.

## How It Works

Two arrays can be broadcast if, when their shapes are right-aligned, each dimension is either equal or one of them is 1. The book lists three permissible cases: same dimensions, one array has a single element, and array-with-scalar. The smaller operand conceptually replicates without actually copying data.

## Key Parameters

- Shapes of operands (right-aligned dimensions)
- Size-1 dimensions used for stretching
- Operator (any element-wise arithmetic / comparison)

## When To Use

- Adding a vector to every row of a matrix
- Scaling arrays by a scalar
- Element-wise math without writing Python loops

## Risks & Pitfalls

- Incompatible shapes raise broadcasting errors
- Silent broadcasting can hide intent in complex pipelines
- Memory cost of intermediate broadcast results

## Related Concepts

- [[concepts/vectorization]]
- [[concepts/ndarray]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
