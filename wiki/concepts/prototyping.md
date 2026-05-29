---
title: Prototyping
type: claim
id: concepts/prototyping
tags:
- foundational
- software-design
- dashboard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/04-introduction.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Prototyping is an iterative software-design approach where functionality is prioritized over polish, brute-force solutions are accepted in early cycles, and code is repeatedly refactored as the problem clarifies. The goal is to converge on a working artifact through cycles of test, abandon, and try again, rather than getting the architecture "right" up front.

## How It Works

In prototyping, early code is allowed to be ugly: bloated classes, commented-out dead branches, and inline values that "just work." Periodic refactoring redistributes the bloated parts into smaller classes/methods. Documentation is sparse — the code is expected to be self-documenting through naming. The author of *Prototyping Python Dashboards* keeps a hardcover paper notebook with a table of contents and dated entries to manage the prototyping log alongside the code.

## Key Parameters

- Iteration cadence (how often to refactor)
- Acceptable level of "ugliness" before refactor
- Note-taking method (paper notebook vs. git history)
- Folder-level versioning (manual copies of working folder) before adopting Git

## When To Use

- Early-stage exploration of an unfamiliar dataset or problem
- Building dashboards or research tools where requirements emerge through use
- Solo or small-team research/academic work
- Tightly time-boxed exploratory work

## Risks & Pitfalls

- Skipping refactoring cycles leaves accumulated complexity that becomes unmaintainable
- Treating the prototype as the final product without hardening it for deployment
- Losing track of which folder copy is "current" without good notes
- Over-engineering early when requirements are still volatile

## Related Concepts

- [[concepts/object-oriented-design]]
- [[concepts/dashboard]]
- [[concepts/python]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
