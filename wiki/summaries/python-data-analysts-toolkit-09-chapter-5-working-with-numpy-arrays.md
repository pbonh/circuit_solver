---
title: 'Python Data Analyst''s Toolkit — Chapter 5: Working with NumPy Arrays'
type: source
id: source-python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays
kind: derived-summary
tags:
- python
- numpy
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt
---

## Key Points

- NumPy ("Numerical Python") provides the `ndarray` data structure, supporting multidimensional homogeneous arrays — overcoming Python's lack of native multidimensional containers.
- Common array constructors: `np.array`, `np.arange`, `np.linspace`, `np.zeros`, `np.ones`, `np.full`, `np.empty`, `np.repeat`, and `np.random.randint`.
- Arrays are homogeneous (single dtype); reshaping is done with `arr.reshape(...)` (returns a new view) or the `shape` attribute (mutates), and `arr.ravel()` flattens back to 1-D.
- An n-dimensional array has n axes (axis=0 is rows, axis=1 is columns, etc.); memory storage is contiguous regardless of logical shape.
- Array length is fixed at creation — elements cannot be appended in place, but values can be reassigned.
- Three array-combining strategies: `np.append`, `np.concatenate` (vertical default; `axis=1` for horizontal), and stacking via `np.vstack` / `np.hstack`.
- Logical operations on arrays use `&`, `|`, `~` (vectorized); `np.all`, `np.any`, and `np.where` test or extract indices/values that satisfy conditions.
- Broadcasting allows arithmetic between arrays of compatible shapes (matching dims, one-element arrays, or array-and-scalar); vectorization applies an operator to each element without explicit loops.
- Dot product (`np.dot`) is distinct from element-wise multiplication (`*` / `np.multiply`).
- Array attributes: `size` (element count), `ndim` (dimensions), `nbytes` (memory), `dtype` (element type), `T` and `np.transpose` (transpose). `type(arr)` returns `numpy.ndarray`; `arr.dtype` returns element type.
- `numpy.ma.masked_array` supports missing/invalid entries via parallel mask arrays.
- Slicing returns a copy and supports row/column/conditional indexing such as `x[:,1]`, `x[3,0]`, `x[x<5]`.
- Aggregate statistics are available as methods or functions: `mean`, `var`, `std`, `sum(axis=...)`, `cumsum`, `max`.
- `np.matrix` provides a 2-D-only subclass where `*` is the dot product; the book recommends preferring `ndarray` since `matrix` may be deprecated.

## Relevant Concepts

- [[concepts/ndarray]] — the core NumPy data structure.
- [[concepts/array-reshaping]] — changing dimensionality of arrays.
- [[concepts/broadcasting]] — rules for combining arrays of compatible shapes.
- [[concepts/vectorization]] — applying operations to whole arrays without loops.
- [[concepts/array-slicing]] — extracting subsets from arrays.
- [[concepts/aggregate-statistics]] — mean/var/std/sum/cumsum over arrays.
- [[concepts/dot-product]] — inner-product operation distinct from element-wise.
- [[concepts/masked-array]] — array with parallel mask indicating invalid entries.
- [[entities/numpy]] — the library introduced and used throughout.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 5 — Working with NumPy Arrays
- File path: raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt
- Author: Gayathri Rajagopalan
