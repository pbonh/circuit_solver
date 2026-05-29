---
title: PEP 8 — Python Style Guide
type: claim
id: concepts/pep-8
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

PEP 8 is the official Python Enhancement Proposal that codifies style guidelines for Python code. Its central goal is readability — anyone reading the code should be able to follow what it does.

## How It Works

PEP 8 specifies conventions for indentation (four spaces, no tabs), line length (79 chars; 72 for comments), naming (lowercase with underscores for functions/modules/variables, UPPER for constants, CapWords for classes), imports (one per line, no wildcards, absolute preferred), comments (prefer block comments; if inline, separate with two spaces), and encoding (UTF-8 in Python 3).

## Key Parameters

- Indent width (4)
- Maximum line length (79 / 72)
- Naming conventions per identifier type
- Import organization

## When To Use

- All Python projects intended for shared or long-term use
- Whenever consistency aids reviewers and tools

## Risks & Pitfalls

- Treating PEP 8 as rigid law instead of a guide
- Mixing tabs and spaces breaks indentation
- Wildcard imports cause name clashes

## Related Concepts

- [[concepts/python]]
- [[concepts/python-modules]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
