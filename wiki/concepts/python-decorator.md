---
title: Python Decorator
type: claim
id: concepts/python-decorator
tags:
- python
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Python decorator is a callable that takes a function and returns a (usually enhanced) function. It is applied with `@decorator_name` syntax placed immediately above a `def` statement, and is the language mechanism behind Dash's `@callback`.

## How It Works

Decorators consume the function they wrap and may return a new function that adds behavior before/after the original call, transforms arguments or return values, or registers the function in some framework registry. Decorators preserve normal call syntax: after `@multiply_these` is applied to `a_times_b()`, calling `a_times_b(2, 3)` still works but goes through the wrapper's added logic.

## Key Parameters

- Wrapped function (positional arg 1 of the decorator's outer call)
- Optional decorator arguments (when written as `@deco(args)` the decorator is itself a factory)
- Use of `functools.wraps` to preserve `__name__`, `__doc__`, etc.

## When To Use

- Registering functions with a framework (Flask routes, Dash callbacks)
- Adding cross-cutting concerns (logging, timing, retries, auth)
- Memoizing return values
- Adapting function signatures

## Risks & Pitfalls

- Without `functools.wraps`, decorated functions lose their introspection metadata
- Stacking many decorators makes tracebacks harder to read
- Decorators that change argument shape can surprise callers

## Related Concepts

- [[concepts/callback]]
- [[concepts/python]]
- [[concepts/reactive-programming]]

## Sources

- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
