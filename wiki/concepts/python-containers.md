---
title: Python Containers
type: claim
id: concepts/python-containers
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Python containers are built-in iterable objects that hold multiple values. The book covers four: lists (mutable, ordered), tuples (immutable, ordered), dictionaries (mutable, unordered key/value pairs), and sets (mutable, unordered unique values).

## How It Works

Each container has its own literal syntax (`[]`, `()`, `{}` with `:`, `{}`) and methods. Lists support `append`/`insert`/`extend`/`remove`/`pop`/`sort`. Tuples support `count`/`index` and unpacking. Dicts support `keys`/`values`/`items`/`get`/`setdefault`. Sets support `add`/`update`/`remove`/`discard`. All four support `len`, iteration via `for`, and slicing (where ordered).

## Key Parameters

- Mutability (mutable vs. immutable)
- Ordering (sequential vs. unordered)
- Duplicate handling (sets remove duplicates)
- Indexing scheme (positional vs. keyed vs. none)

## When To Use

- Lists for ordered collections of similar items
- Tuples for fixed-size heterogeneous records
- Dictionaries for keyed lookup
- Sets for membership tests and de-duplication

## Risks & Pitfalls

- Mutating a list while iterating
- Treating tuples as lists (they cannot be modified)
- Forgetting that dicts/sets are unordered (historically)
- Mixing types in a list that is later sorted

## Related Concepts

- [[concepts/list-comprehension]]
- [[concepts/python]]
- [[concepts/control-flow]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
