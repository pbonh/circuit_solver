---
title: "Encapsulation"
type: concept
tags: [python, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt"]
confidence: medium
---

## Definition

Encapsulation is the OOP principle of binding data (instance variables) with the methods that operate on it, and protecting that data from arbitrary outside manipulation. Once an object is created, its variables are accessed and modified through its methods.

## How It Works

In Python, attributes belong to instances and are reached only through the object reference (`obj.attribute`). The chapter illustrates that the bare variable name (e.g., `radius` instead of `c.radius`) is out of scope outside the class. Methods provide a controlled interface to read or change state.

## Key Parameters

- Visibility conventions (single underscore for "internal")
- Whether attributes are read-only via properties
- Method interface granularity

## When To Use

- Whenever a class owns state that should not be mutated globally
- To present a stable API while letting internals evolve

## Risks & Pitfalls

- Python does not enforce truly private attributes
- Leaky abstractions through public access to mutable internals

## Related Concepts

- [[concepts/object-oriented-programming]]
- [[concepts/data-abstraction]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
