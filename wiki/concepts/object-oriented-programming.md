---
title: "Object-Oriented Programming"
type: concept
tags: [python, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/06-chapter-2-exploring-containers-classes-and-objects.txt"]
confidence: high
---

## Definition

Object-oriented programming (OOP) is a paradigm that organizes code around classes and objects rather than procedural sequences of steps. Python is an OOP language; classes are blueprints and objects are instances.

## How It Works

A class declared with the `class` keyword groups class variables, an `__init__` constructor, and methods. The first parameter of any instance method is `self`, which refers to the calling instance. Objects are created by calling the class with constructor arguments. The four pillars are encapsulation, polymorphism, inheritance, and data abstraction.

## Key Parameters

- Class variables (shared) vs. instance variables (per-object)
- `__init__` constructor signature
- Method `self` parameter
- Parent class for inheritance

## When To Use

- Modeling stateful entities with behavior
- Avoiding global variables and accidental manipulation
- Building reusable, hierarchical components

## Risks & Pitfalls

- Overengineering simple scripts with class hierarchies
- Deep inheritance trees create fragile coupling
- Mutable class variables shared across instances cause subtle bugs

## Related Concepts

- [[concepts/encapsulation]]
- [[concepts/polymorphism]]
- [[concepts/inheritance]]
- [[concepts/data-abstraction]]
- [[concepts/python]]

## Sources

- [[summaries/python-data-analysts-toolkit-06-chapter-2-exploring-containers-classes-and-objects]]
