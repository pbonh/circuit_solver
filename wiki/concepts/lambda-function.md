---
title: "Lambda Function"
type: concept
tags: [python, functional-programming, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/DataAnalysisAndVisualizationsPython/_txt/04-chapter-1-introduction-to-data-science-with-python.txt"]
confidence: medium
---

## Definition

A lambda function in Python is a short anonymous function created with the `lambda` keyword. It returns the value of a single expression and is commonly used with `map`, `filter`, and `reduce`.

## How It Works

The syntax is `lambda arg1, arg2, ...: expression`. The result is a callable object that takes the listed arguments and returns the expression's value. Lambdas are first-class values that can be assigned to variables, passed into higher-order functions, or used inline as throwaway computations.

## Key Parameters

- Argument list (any number, optionally with defaults)
- A single expression body (no statements)

## When To Use

- Short callbacks passed to `map`/`filter`/`reduce` or sorting `key=` arguments
- Inline transformations within a Pandas `.apply()` call
- Quick numerical or boolean predicates that don't merit a `def`

## Risks & Pitfalls

- Cannot contain statements or multiple expressions
- Overuse harms readability versus named `def` functions
- Late binding of free variables can cause subtle closure bugs

## Related Concepts

- [[concepts/python]]
- [[concepts/data-analysis]]

## Sources

- [[summaries/data-analysis-visualizations-python-04-chapter-1-introduction-to-data-science-with-python]]
- [[summaries/data-analysis-visualizations-python-09-chapter-6-data-exploring-and-analysis]]
