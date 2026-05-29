---
title: Exception Handling (Python)
type: claim
id: concepts/exception-handling
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Exception handling is the mechanism Python uses to respond to runtime errors without crashing the program. The `try` block contains code that may raise; the `except` block catches matching exception types and runs recovery code.

## How It Works

All exceptions derive from `BaseException` and follow a class hierarchy. Code that could fail (for example, `int(input(...))`) is placed in a `try` block; matching `except` clauses are listed below by exception class (`ValueError`, `ZeroDivisionError`, etc.) and execute when an exception of that type propagates. A user-friendly message inside the `except` block lets the user correct the input.

## Key Parameters

- Exception class to catch
- Number and order of `except` clauses
- Optional `else` and `finally` blocks
- Re-raising via bare `raise`

## When To Use

- Validating user input
- Wrapping file or network I/O that may fail
- Converting low-level errors into domain-specific exceptions

## Risks & Pitfalls

- Catching `Exception` (or worse, bare `except:`) hides bugs
- Swallowing errors silently makes debugging hard
- Resource leaks if you don't use `with`/`finally`

## Related Concepts

- [[concepts/control-flow]]
- [[concepts/file-io]]
- [[concepts/python]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
