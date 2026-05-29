---
title: Regular Expression
type: claim
id: claim-regular-expression
tags:
- python
- regex
- data-cleaning
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
confidence:
  base: 0.85
---

## Definition

A regular expression (regex) is a compact pattern syntax for matching and transforming substrings within a text. Python's `re` module provides functions like `re.sub(pattern, replacement, text)` used by the book's CSV cleanup pass to strip HTML, embedded commas, and stray whitespace.

## How It Works

A regex is compiled into an automaton that, when applied to a target string, finds matches or replaces them. Common metacharacters: `[A-Z]` (character class), `{n}` (exact repetition), `( )` (capturing groups), `\1` (back-reference), `|` (alternation), `?` (optional), `+` / `*` (one-or-more / zero-or-more). The book uses `re.sub(r'([A-Z]{3})([ ])', r'\1', line0, count=1)` to remove the trailing space after a three-letter airport code, capturing the code and discarding the space.

## Key Parameters

- Pattern (raw string `r"..."` to avoid backslash escaping)
- Replacement (may use back-references `\1`)
- Flags (`re.IGNORECASE`, `re.MULTILINE`, `re.DOTALL`)
- `count` (limits the number of substitutions)

## When To Use

- Stripping HTML tags and embedded markup
- Normalizing whitespace and number formats during data cleaning
- Validating user input shapes (dates, phone numbers, IDs)
- Extracting structured pieces from log lines

## Risks & Pitfalls

- Patterns are read-only; complex regexes become hard to maintain — comment them
- Greedy vs. lazy quantifiers can match more than intended
- Regex is not the right tool for parsing nested structures like full HTML
- Performance pathology with catastrophic backtracking on adversarial inputs

## Related Concepts

- [[concepts/regular-expressions]]
- [[concepts/data-cleaning]]
- [[concepts/csv]]
- [[concepts/python]]

## Sources

- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
