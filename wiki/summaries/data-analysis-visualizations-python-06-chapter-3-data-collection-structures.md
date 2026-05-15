---
title: "Data Analysis and Visualizations with Python — Chapter 3: Data Collection Structures"
type: summary
tags: [python, pandas, data-structures, dataframe, series, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/DataAnalysisAndVisualizationsPython/_txt/06-chapter-3-data-collection-structures.txt"]
confidence: high
---

## Key Points

- Surveys the six core Python data-collection structures used in data science: lists, dictionaries, tuples, Pandas Series, Pandas DataFrames, and Pandas Panels.
- Lists: mutable ordered sequences in `[]`; supports forward and backward indexing, slicing, `+`/`*` concatenation/repetition, membership via `in`, and updates by index assignment.
- List mutation: `append` (single element), `extend` (concatenate another list), `insert(index, obj)`, `del list[i]`, `remove(obj)` (first occurrence), and `pop(i)` (remove and return).
- List functions and methods: `cmp`, `len`, `max`, `min`, `sum`, `list(seq)`, `sort`, `reverse`, `index`, `count`; converting between strings and lists with `list(word)`, `split(delim)`, and `delimiter.join(list)`.
- Aliasing pitfall: `b = a` makes `b` reference the same list as `a`, so mutations propagate; `b = [1,2,3]` creates an independent list.
- Dictionaries: unordered key-value pairs in `{}` with unique immutable keys; created via literals or `dict()`; mutated by key assignment; deleted with `del d[key]`, `d.clear()`, or `del d`.
- Dictionary methods: `get`, `items`, `keys`, `values`, `setdefault`, `update`, `fromkeys`, `copy`; sortable by key with `sorted(d)` or by value with `sorted(d, key=d.get)`.
- Tuples: immutable ordered sequences in `()`; support indexing, slicing, concatenation, repetition, membership, and iteration but reject element assignment; sortable in place with `.sort(key=lambda x: ...)` or via `sorted(t)`.
- Pandas Series: 1D labeled array constructed from ndarrays, scalars, dicts, or lists; supports label-based and integer-position access, NumPy vectorized ops, alignment-based arithmetic (mismatched labels yield NaN), and a `name` attribute settable via `rename`.
- Pandas DataFrame: 2D labeled tabular structure built from dicts of Series, dicts of ndarrays/lists, structured/record arrays, lists of dicts, or dicts of tuples (multi-index); supports column add/delete via `df[col] = ...`, `del`, `pop`, `insert`, and method-chained derivations via `.assign(C=lambda x: x['A']+x['B'])`.
- DataFrame indexing: `df[col]` (column → Series), `df.loc[label]`, `df.iloc[loc]`, `df[5:10]` (slice rows), `df[bool_vec]` (boolean filter); transpose via `df.T`; matrix multiplication via `df.T.dot(df)`.
- Pandas Panel: 3D container with `items` (axis 0), `major_axis` (rows), `minor_axis` (columns); constructed from 3D ndarrays or dicts of DataFrames; sliced via `panel[item]`, `panel.major_xs(val)`, `panel.minor_xs(val)`.
- Closes with exercises that build a Series of student GPAs and a DataFrame of course grades with a computed Mean column.

## Relevant Concepts

- [[concepts/python]] — language and runtime hosting all structures discussed.
- [[concepts/data-analysis]] — purpose for choosing each structure.
- [[concepts/dataframe]] — central tabular structure introduced in this chapter.
- [[concepts/missing-data-handling]] — NaN appears whenever Series labels don't align.
- [[entities/pandas]] — provides Series, DataFrame, and Panel.
- [[entities/numpy]] — provides ndarrays that back Series and DataFrame construction.

## Source Metadata

- Source type: book chapter
- Book title: Data Analysis and Visualizations with Python
- Chapter: 3 — Data Collection Structures
- File path: raw/DataAnalysisAndVisualizationsPython/_txt/06-chapter-3-data-collection-structures.txt
- Author: Ossama Embarak
