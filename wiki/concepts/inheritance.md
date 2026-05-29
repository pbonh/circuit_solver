---
title: Inheritance
type: claim
id: concepts/inheritance
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Inheritance is the OOP mechanism of creating a child class from a parent class. The child inherits attributes and methods from the parent and may add its own. Python uses `class Child(Parent):` syntax, with `pass` placeholder for empty bodies.

## How It Works

The chapter shows a `Mother` parent with an `__init__` and a `nameprint` method, and a `Child(Mother)` class that inherits everything via `pass`. Child classes can override methods or extend behavior. Multiple inheritance is allowed but not detailed here.

## Key Parameters

- Parent class(es)
- Overridden methods
- Use of `super()` for delegating to the parent
- Method resolution order (MRO)

## When To Use

- Sharing behavior across closely related classes
- Specializing a generic base class
- Building plugin-style hierarchies

## Risks & Pitfalls

- Deep hierarchies are brittle
- "Is-a" relationships often misapplied — prefer composition where sensible
- Multiple inheritance can introduce diamond ambiguities

## Related Concepts

- [[concepts/object-oriented-programming]]
- [[concepts/polymorphism]]
- [[concepts/encapsulation]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
