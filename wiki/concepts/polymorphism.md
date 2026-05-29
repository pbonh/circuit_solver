---
title: Polymorphism
type: claim
id: claim-polymorphism
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt
confidence:
  base: 0.65
---

## Definition

Polymorphism — "one interface, many forms" — is the ability to use the same method or function regardless of the underlying data type. The chapter illustrates this with `len()` working on strings, lists, tuples, and dictionaries.

## How It Works

Python implements polymorphism via duck typing: any type that supplies the expected method (e.g., `__len__`) plays the role. There is no need to write a different `len_string`, `len_list`, etc. Operator overloading (e.g., `+` for strings, lists, numbers) is another form.

## Key Parameters

- Required interface (method/operator names)
- Whether dispatch is by inheritance or by duck typing
- Fallback behavior when the type does not satisfy the interface

## When To Use

- Writing generic utilities that operate on any compatible type
- Designing APIs that accept multiple data structures

## Risks & Pitfalls

- Silent failures when a type only partially implements the interface
- Hard-to-trace errors at runtime when types differ from expectations

## Related Concepts

- [[concepts/object-oriented-programming]]
- [[concepts/inheritance]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
