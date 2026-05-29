---
title: Dot Product
type: claim
id: concepts/dot-product
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
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The dot product is the inner-product operation on two arrays — for 1-D vectors, the sum of element-wise products; for 2-D matrices, ordinary matrix multiplication. NumPy provides `np.dot(a, b)` (and `a @ b` via the `@` operator).

## How It Works

Element-wise multiplication (`*` or `np.multiply`) preserves shape and operates entry by entry. The dot product instead contracts the inner dimension — so a 3×2 matrix times a 2×3 matrix yields a 3×3 result. The book contrasts the two operations side by side.

## Key Parameters

- Shapes (inner dimensions must match)
- Use of `np.matmul` / `@` vs. `np.dot`
- dtype precision

## When To Use

- Matrix multiplication, projection, similarity
- Neural-network layer computations
- Linear-algebra building block

## Risks & Pitfalls

- Confusing `*` with `@`
- Forgetting shape compatibility rules
- `np.matrix` overloads `*` differently than `ndarray`

## Related Concepts

- [[concepts/ndarray]]
- [[concepts/vectorization]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
