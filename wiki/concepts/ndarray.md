---
title: "NumPy ndarray"
type: concept
tags: [python, numpy, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt"]
confidence: high
---

## Definition

The `ndarray` is NumPy's homogeneous, n-dimensional array data structure. It holds elements of a single dtype, exposes shape and stride metadata, and is the substrate for vectorized numerical computation across the Python data-science stack.

## How It Works

A constructor like `np.array`, `np.arange`, or `np.linspace` returns an `ndarray`. Internally elements occupy contiguous memory regardless of logical shape; axes describe the logical dimensions. Operations are dispatched to highly optimized C/Fortran routines that exploit broadcasting and SIMD.

## Key Parameters

- Shape (tuple of axis sizes)
- dtype (element type)
- Strides (bytes between consecutive elements per axis)
- Memory layout (C vs. F order)

## When To Use

- Any numerical work that benefits from vectorization
- Backing store for Pandas, SciPy, scikit-learn, Matplotlib
- Large numerical datasets in memory

## Risks & Pitfalls

- Length is fixed at creation; appending requires copies
- Mixing dtypes silently promotes to a common one
- Slicing usually returns views, not copies (but the book notes copy semantics for explicit slicing operations)

## Related Concepts

- [[concepts/broadcasting]]
- [[concepts/vectorization]]
- [[concepts/array-reshaping]]
- [[concepts/array-slicing]]
- [[entities/numpy]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
