---
title: 'Python Data Analyst''s Toolkit — Chapter 2: Exploring Containers, Classes,
  and Objects'
type: source
id: summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects
kind: publication
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt
---

## Key Points

- Containers are iterable objects that hold multiple values; Python's four built-in containers are lists, tuples, dictionaries, and sets.
- Lists are mutable ordered sequences; key methods include `append`, `insert`, `extend`, `del`, `remove`, `pop`, `index`, `len`, `sort`, and `reverse`.
- Lists support slicing with start/stop/step (both positive and negative indices); concatenation via `+` produces a new list while `extend` modifies in place.
- New lists can be derived via list comprehensions (`[expr for x in lst if cond]`), the `map` function (transform each element), and the `filter` function (keep elements matching a predicate).
- `zip` pairs elements across multiple lists; `enumerate` yields `(index, item)` tuples.
- Tuples are immutable, ordered, and faster than lists; defined with parentheses (or just commas). Methods include `count` and `index`; supports tuple unpacking (including `*_` for the rest).
- Dictionaries store unordered key/value pairs in curly braces; methods include `keys`, `values`, `items`, `get`, `setdefault`, and `clear`. `get` is non-mutating; `setdefault` writes if missing.
- Sets store unordered collections of unique elements (duplicates are silently dropped); operations include `add`, `update`, `remove`, `discard`, `len`, and iteration.
- Python supports object-oriented programming with classes (`class` keyword), instance variables, class variables, constructor `__init__`, and the `self` parameter on methods.
- OOP principles covered: encapsulation (binding data with methods, hiding internals), polymorphism (one interface, many forms — illustrated by `len` over different types), inheritance (child class derives from parent; `pass` keyword for empty bodies), and data abstraction (exposing behavior, hiding implementation).
- Mutability + ordering matrix: lists (mutable, ordered), tuples (immutable, ordered), dictionaries (mutable, unordered), sets (mutable, unordered).

## Relevant Concepts

- [[concepts/python-containers]] — overview of lists, tuples, dicts, sets.
- [[concepts/list-comprehension]] — concise idiom for building new lists.
- [[concepts/object-oriented-programming]] — paradigm Python follows with classes and objects.
- [[concepts/encapsulation]] — binding data and behavior.
- [[concepts/polymorphism]] — one interface, many forms.
- [[concepts/inheritance]] — deriving child classes from parents.
- [[concepts/data-abstraction]] — exposing behavior, hiding implementation.
- [[concepts/python]] — language whose container and class system this chapter covers.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 2 — Exploring Containers, Classes, and Objects
- File path: raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt
- Author: Gayathri Rajagopalan
