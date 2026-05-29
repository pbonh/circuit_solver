---
title: 'Python Data Analyst''s Toolkit — Chapter 3: Regular Expressions and Math with
  Python'
type: source
id: summaries/python-data-analysts-toolkit-07-chapter-3-regular-expressions-and-math-with-python
kind: publication
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/07-chapter-3-regular-expressions-and-math-with-python.txt
---

## Key Points

- The chapter covers two libraries: `re` for regular expressions and `SymPy` for symbolic math (algebra, calculus, sets, probability).
- A regular expression is a pattern of literal characters and metacharacters used to search, replace, or extract structured text such as dates, postal codes, phone numbers, HTML tags, or email addresses.
- The standard `re` workflow is: import `re`, compile a pattern (optionally prefixed `r''` raw-string), then call `search`/`match`/`findall`/`split`/`sub` on the compiled object or via module-level shortcuts.
- `findall` returns all matches; `search` returns the first match anywhere; `match` only matches at the beginning of the string; `split` partitions on the pattern; `sub` substitutes.
- Metacharacters covered: `.` (any single char), `[]` (character set), `?` (0 or 1), `*` (0 or more), `+` (1 or more), `{m,n}` (range of repetitions), `^` (start of string), `$` (end of string), `\` (escape or character class introduction).
- Character classes: `\d` digit, `\D` non-digit, `\w` alphanumeric, `\W` non-alphanumeric, `\s` whitespace, `\S` non-whitespace; backslash also escapes metacharacters so they match literally.
- SymPy operates on `Symbol` objects representing algebraic variables; functions `factor`, `expand`, `solve`, and `sympify` (for user-entered strings) cover algebra.
- Simultaneous equations are solved by passing a tuple of expressions to `solve`; `sympy.plotting.plot` can visualize them, showing intersection as the solution.
- Sets are created with `FiniteSet`; `union` and `intersect` perform set operations; probability of an event is computed as `len(event)/len(sample_space)` over `FiniteSet`s.
- Calculus is supported via `limit`, `diff` (derivative), and `integrate` (definite or indefinite integral); definite integrals take a `(symbol, a, b)` tuple as the second argument.
- These tools become building blocks for later chapters on data wrangling, visualization, and statistics.

## Relevant Concepts

- [[concepts/regular-expressions]] — pattern language and re-module workflow.
- [[concepts/symbolic-mathematics]] — algebra/calculus/sets via SymPy.
- [[concepts/probability]] — computed here via finite-set sample space.
- [[concepts/calculus]] — limits, derivatives, integrals through SymPy.
- [[concepts/set-theory]] — operations on finite sets.
- [[entities/python-re-module]] — the regex module used in the chapter.
- [[entities/sympy]] — symbolic mathematics library.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 3 — Regular Expressions and Math with Python
- File path: raw/PythonDataAnalystsToolkit/_txt/07-chapter-3-regular-expressions-and-math-with-python.txt
- Author: Gayathri Rajagopalan
