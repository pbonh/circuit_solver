---
title: 'Data Analysis and Visualizations with Python — Chapter 4: File I/O Processing
  and Regular Expressions'
type: source
id: source-data-analysis-visualizations-python-07-chapter-4-file-i-o-processing-and-regular-expressions
kind: derived-summary
tags:
- python
- file-io
- regular-expressions
- data-extraction
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/07-chapter-4-file-i-o-processing-and-regular-expressions.txt
---

## Key Points

- Covers screen-based input/output using `input()` (always returns text — convert with `int()`/`float()`), and formatted output using escape sequences like `\t` and `\n`.
- Describes `open(file_name, access_mode, buffering)` with all twelve open modes: `r`/`rb`/`r+`/`rb+`, `w`/`wb`/`w+`/`wb+`, `a`/`ab`/`a+`/`ab+`.
- File-object attributes: `file.closed`, `file.mode`, `file.name`; closing via `file.close()` to flush buffers and release the OS handle.
- File operations: `file.read()`, `file.write(string)`, `os.rename(old, new)`, `os.remove(name)`.
- Directory operations via `os` module: `os.mkdir`, `os.chdir`, `os.getcwd`, `os.rmdir`.
- Introduces regular expressions as a special pattern-matching language for text extraction across files, XML, JSON, and HTML.
- Comprehensive table of Python regex syntax: anchors (`^`, `$`, `\A`, `\Z`, `\b`), character classes (`.`, `[...]`, `[^...]`, `\d`, `\D`, `\w`, `\W`, `\s`, `\S`), quantifiers (`*`, `+`, `?`, `{n}`, `{n,}`, `{n,m}`), alternation (`a|b`), groups (`(re)`, `(?:re)`), lookahead/lookbehind (`(?=re)`, `(?!re)`), and backreferences (`\1`–`\9`).
- Demonstrates extracting `From:` lines and email addresses from a mail log file both via string methods (`startswith`, `split`) and via `re.findall`.
- Distinguishes greedy (`^F.+:`) from non-greedy (`^F.+?:`) matching with a `'From: Using the : character'` example.
- Demonstrates `re.findall('@([^ ]*)', mystr)` to extract hostnames from email addresses and `re.findall('\$[0-9.]+', mystr)` to extract dollar amounts (using `\$` to escape the literal `$`).
- Tables for alternatives (`python|RLang`), repetition (`ruby?`, `\d{3,5}`), and anchors with practical examples.
- End-of-chapter exercises parse a multi-line course catalog into tuples of (number, code, name) using `([0-9]+)\s*([A-Z]{3})\s*([A-Za-z]{4,})`.

## Relevant Concepts

- [[concepts/python]] — host language and runtime.
- [[concepts/regular-expressions]] — central topic of the chapter.
- [[concepts/data-analysis]] — file ingestion is the entry point of any analysis.
- [[concepts/data-extraction]] — extracting structured information from text and logs.

## Source Metadata

- Source type: book chapter
- Book title: Data Analysis and Visualizations with Python
- Chapter: 4 — File I/O Processing and Regular Expressions
- File path: raw/DataAnalysisAndVisualizationsPython/_txt/07-chapter-4-file-i-o-processing-and-regular-expressions.txt
- Author: Ossama Embarak
