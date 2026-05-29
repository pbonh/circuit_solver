---
title: List Comprehension
type: claim
id: concepts/list-comprehension
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

A list comprehension is a concise Python expression for building a new list from an existing iterable by combining an output expression, a `for` clause, and an optional `if` filter: `[expr for x in iterable if cond]`.

## How It Works

The interpreter iterates over the source iterable, evaluates the output expression for each element (filtered by the condition if present), and collects results into a new list. Equivalent imperative loops are longer; list comprehensions read declaratively when small.

## Key Parameters

- Output expression
- Loop variable(s) and source iterable(s)
- Optional filter predicate
- Optional conditional output (`x if cond else y`)

## When To Use

- Mapping and filtering simple list transformations in one line
- Replacing short loops that build a list
- Constructing lookup tables (with dict/set comprehensions analogously)

## Risks & Pitfalls

- Overly nested or multi-condition comprehensions hurt readability
- Memory use for large lists (consider generators)
- Side effects in the expression are surprising to readers

## Related Concepts

- [[concepts/python-containers]]
- [[concepts/python-functions]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
