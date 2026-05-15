---
title: "Python Functions"
type: concept
tags: [python, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt"]
confidence: high
---

## Definition

A Python function is a reusable, named block of statements defined with `def` that accepts zero or more parameters, optionally returns a value, and has its own local scope. Lambda (anonymous) functions are single-expression functions defined with the `lambda` keyword.

## How It Works

`def name(params):` introduces a function; parameters become locally scoped names. The body executes when the function is called; `return` exits with a value. Variables defined inside the function are not visible outside it. Lambdas have an implicit return and are useful as small inline callbacks.

## Key Parameters

- Parameter list (positional, keyword, default, `*args`, `**kwargs`)
- Return value (single object or tuple)
- Local vs. enclosing vs. global scope
- Lambda vs. `def`

## When To Use

- Eliminating repeated code blocks
- Encapsulating reusable logic
- Passing behavior as a callback (lambdas)

## Risks & Pitfalls

- Mutable default arguments shared across calls
- Modifying enclosing-scope state implicitly
- Overusing lambdas hurts readability

## Related Concepts

- [[concepts/python]]
- [[concepts/control-flow]]
- [[concepts/python-modules]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
