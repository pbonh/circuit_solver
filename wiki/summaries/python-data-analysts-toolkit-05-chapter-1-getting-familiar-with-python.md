---
title: 'Python Data Analyst''s Toolkit — Chapter 1: Getting Familiar with Python'
type: source
id: summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python
kind: publication
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt
---

## Key Points

- Python is an open-source, interpreted, high-level language created by Guido van Rossum and is the lingua franca of data science and machine learning.
- The chapter recommends installing Anaconda to get Python, Jupyter, and hundreds of libraries at once; code examples target Python 3.7.3 and Anaconda 4.7.10.
- Jupyter notebooks are the IDE of choice; they support code, markdown, tab completion, keyboard shortcuts, and magic commands like `%matplotlib inline` and `%%timeit`.
- Python uses four-space indentation (no semicolons, no curly braces) and supports comments with `#`. Variables are dynamically typed; rules forbid leading digits, spaces, and reserved keywords in names.
- The chapter surveys operators (arithmetic, comparison, logical, assignment, identity, membership) and basic data types (int, float, str, bool, list, tuple, range, dict).
- The `datetime` module supplies `date`, `time`, `datetime`, and `timedelta`; `timedelta` can be added to dates and datetimes but not to time objects.
- Strings are immutable sequences; the chapter covers indexing, slicing, justification, case changes, content checks, joining, and splitting.
- Conditional execution uses `if`/`elif`/`else` (no switch-case); iteration uses `while` and `for` loops, with `break` and `continue` for control flow.
- Functions are defined with `def`; lambda (anonymous) functions provide single-expression alternatives. Local-scoped variables exist only inside functions.
- Exceptions are handled with `try`/`except` (all derived from `BaseException`); user-friendly messages make programs more robust to invalid input.
- File I/O uses `open` with modes `r`, `w`, `a`, `w+`; the `csv` module's `reader`/`writer` objects handle CSV files line by line.
- Modules are `.py` files importable via `import`/`from ... import ...`. PEP 8 codifies style rules: 4-space indent, 79-char lines, no wildcard imports, descriptive lowercase names, CapWords for classes, UTF-8 encoding.

## Relevant Concepts

- [[concepts/python]] — the language whose syntax, types, and built-ins this chapter introduces.
- [[concepts/control-flow]] — conditional statements and loops as covered here.
- [[concepts/exception-handling]] — try/except construct for handling runtime errors.
- [[concepts/python-functions]] — `def` and lambda functions, return values, scope.
- [[concepts/python-modules]] — organizing code into `.py` files for reuse.
- [[concepts/pep-8]] — Python style guide summarized in the chapter.
- [[concepts/file-io]] — reading/writing files and CSV handling.
- [[entities/anaconda]] — Python distribution installed at the start of the chapter.
- [[entities/jupyter-notebook]] — interactive environment used throughout.
- [[entities/python-datetime-module]] — module covered for handling dates and times.
- [[entities/python-csv-module]] — module covered for reading/writing CSV files.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 1 — Getting Familiar with Python
- File path: raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt
- Author: Gayathri Rajagopalan
