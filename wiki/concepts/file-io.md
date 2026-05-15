---
title: "File I/O (Python)"
type: concept
tags: [python, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt"]
confidence: high
---

## Definition

File I/O is reading from and writing to files on disk from Python. The `open` built-in returns a file object; the `with` statement ensures it's closed automatically. The `csv` module wraps reading/writing CSV files with `csv.reader` and `csv.writer`.

## How It Works

`open(path, mode)` accepts modes `r` (read), `w` (overwrite), `a` (append), `w+` (read/write). The `with open(...) as f:` idiom guarantees `f.close()` even on exceptions. For CSVs, `csv.reader(f)` yields rows as lists; `csv.writer(f).writerow([...])` appends a row.

## Key Parameters

- Path (absolute or relative)
- Mode (`r`, `w`, `a`, `w+`, `b` suffix for binary)
- Encoding (defaults to platform-specific text)
- Delimiter for CSV files

## When To Use

- Loading raw data for analysis
- Persisting results to disk
- Streaming through large files line by line

## Risks & Pitfalls

- Forgetting to close files (use `with`)
- Overwriting with `w` mode unintentionally
- Encoding mismatches on cross-platform data

## Related Concepts

- [[concepts/python]]
- [[concepts/exception-handling]]
- [[entities/python-csv-module]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
