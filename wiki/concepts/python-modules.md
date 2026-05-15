---
title: "Python Modules"
type: concept
tags: [python, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt"]
confidence: high
---

## Definition

A Python module is a `.py` file that groups related functions, classes, and variables. Modules are imported with the `import` keyword (or `from ... import ...`) and may nest sub-modules. The standard library plus the third-party ecosystem (NumPy, Pandas, Matplotlib, etc.) are exposed as modules.

## How It Works

When Python imports a module it executes the file's top-level code once and binds the result to the import name. `from x import y` pulls just one attribute. Custom modules sit in the same directory as the importing script (or anywhere on `sys.path`).

## Key Parameters

- Module file name (becomes the import name)
- `__init__.py` for packages
- `sys.path` search order
- Absolute vs. relative imports

## When To Use

- Splitting a large script into reusable units
- Sharing code across notebooks or projects
- Encapsulating third-party libraries

## Risks & Pitfalls

- Wildcard imports pollute the namespace (PEP 8 advises against)
- Circular imports
- Slow imports if heavy top-level code runs

## Related Concepts

- [[concepts/python]]
- [[concepts/python-functions]]
- [[concepts/pep-8]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
