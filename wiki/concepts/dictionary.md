---
title: Python Dictionary
type: claim
id: claim-dictionary
tags:
- python
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt
confidence:
  base: 0.85
---

## Definition

A Python dictionary is a key/value mapping written with curly braces `{}` and accessed by key. Dictionaries are foundational for building Pandas DataFrames (each key becomes a column name) and for mapping user-friendly labels to internal variable names in Dash dropdowns.

## How It Works

Each dictionary entry has a hashable key and an associated value. Access uses `d[key]`; iteration yields keys; `d.keys()`, `d.values()`, and `d.items()` return views. Notably, `d.keys()` returns a non-indexable set-like view; wrap with `list(...)` to allow `[i]` access.

## Key Parameters

- Hashable keys (strings, numbers, tuples of immutables)
- Insertion order preserved (Python 3.7+)
- Mutability of values

## When To Use

- Looking up values by name (e.g., menu-label → internal column name)
- Constructing a Pandas DataFrame from named columns
- Counting occurrences (with `dict.get(k, 0) + 1` or `collections.Counter`)
- Holding structured configuration

## Risks & Pitfalls

- `keys()` is not directly indexable
- Missing-key access raises `KeyError` unless you use `.get()` or `setdefault()`
- Mutable values shared across keys can yield aliasing bugs

## Related Concepts

- [[concepts/python]]
- [[concepts/list]]
- [[concepts/dataframe]]
- [[entities/pandas]]

## Sources

- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
