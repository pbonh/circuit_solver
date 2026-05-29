---
title: Object-Oriented Design
type: claim
id: concepts/object-oriented-design
tags:
- foundational
- software-design
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Object-Oriented Design (OOD) organizes software into classes that bundle data with the methods that act on it. Each class compartmentalizes a portion of the program's behavior and state, which scales better than passing dozens of globals or arguments through linear procedural code.

## How It Works

A class declaration defines fields and methods; instantiating the class creates an object that holds its own copies of fields and on which methods can be invoked. Class variables (`self.x`) act as a "locally global" namespace shared across the class's methods. Classes can inherit from each other (Auto, Bike, Train, Ship all derive from a Transport class) to share behavior without duplication.

## Key Parameters

- Class boundary (what data and methods belong together)
- Public interface vs. internal helpers
- Inheritance hierarchies
- Constructor (`__init__`) responsibilities

## When To Use

- Mid-size and larger projects where related state and behavior need to travel together
- Code intended to be shared with other team members
- Dashboards split across multiple files (layout, data, figures)
- Long-lived projects whose first prototype was linear but now needs structure

## Risks & Pitfalls

- Over-engineering tiny utilities into class hierarchies
- "God classes" that grow to handle everything and become unmaintainable
- Inheritance that obscures behavior; composition is often clearer
- Premature abstraction before requirements are stable

## Related Concepts

- [[concepts/object-oriented-programming]]
- [[concepts/encapsulation]]
- [[concepts/inheritance]]
- [[concepts/python]]
- [[concepts/dashboard]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
