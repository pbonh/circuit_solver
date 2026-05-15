---
title: "Prototyping Python Dashboards — Chapter 1: Working with Python"
type: summary
tags: [python, dataframe, pandas, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt"]
confidence: high
---

## Key Points

- Python has nuanced data structures whose quirks (e.g., `range(1,5)` returning `[1,2,3,4]`) can surprise beginners; appreciating data-type behavior prevents many debugging walls.
- The chapter contrasts linear FORTRAN-style coding with event-driven (reactive) programming, motivating Object-Oriented Design (OOD) as a way to manage GUI/dashboard complexity through classes that bundle data with methods.
- Tuples (`()`) are immutable ordered collections; sets (`{}`) are unordered collections of unique elements that cannot be indexed; lists (`[]`) are ordered, indexable, and mutable, and accept duplicates.
- Copying a list via `B = A` makes a reference; use `B = A.copy()` to make a real copy. Shallow vs. deep copies trade speed for safety.
- Dictionaries use key/value pairs; `dict.keys()` returns a set, not a list — wrap with `list(...)` to index.
- Pandas Series support label and integer indexing, vector arithmetic, and combination.
- Pandas DataFrames are 2D mixed-type structures, built from lists or dictionaries; building from dictionaries is more predictable because keys become column names while building from lists makes the last list the index by default.
- DataFrame access patterns: `df.col2` or `df['col2']` returns a Series, which can be converted to a plain list with `list(...)`; `loc[]` accesses by label, `iloc[]` by integer position, and both can take row/column specifiers including non-contiguous lists.
- Boolean filtering: `df[df.age > 20]` extracts matching rows; filters combine via `&` and `|` (always parenthesize for safe parsing).
- The Spyder IDE's variable explorer is highlighted as essential for inspecting dataframe shape, dtype, and contents during prototyping.

## Relevant Concepts

- [[concepts/python]] — the language whose data structures the chapter surveys.
- [[concepts/dataframe]] — the central data structure for the rest of the book.
- [[entities/pandas]] — library providing Series and DataFrame.
- [[concepts/object-oriented-design]] — recommended paradigm for organizing dashboard code.
- [[concepts/reactive-programming]] — motivation for OOD given GUI-driven event handling.
- [[concepts/list]] — Python's ordered mutable collection, used pervasively for dashboard data.
- [[concepts/dictionary]] — key/value mapping used for column names and dataframe construction.
- [[concepts/series]] — Pandas 1D labeled array that DataFrame columns are instances of.
- [[entities/spyder-ide]] — the IDE the author uses, featuring the variable explorer.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 1 — Working with Python
- File path: raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt
- Author: Padraig Houlahan
