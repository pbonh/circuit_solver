---
title: CSV
type: claim
id: claim-csv
tags:
- data-format
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt
confidence:
  base: 0.85
---

## Definition

CSV (comma-separated values) is a plain-text tabular file format in which each line is a row and fields are separated by commas (or another delimiter). CSV files are widely supported, compact compared with HTML or Excel, and a natural input for pandas `read_csv()`.

## How It Works

Each CSV row encodes one record; the first row often holds column headers. Fields can be quoted (typically with double quotes) when they contain the delimiter or newlines. Cleaning real-world data before saving as CSV usually requires stripping markup, removing embedded thousands-separator commas in numbers (e.g., turning `1,036` into `1036`), and normalizing whitespace.

## Key Parameters

- Delimiter (`,`, `;`, `\t`, ...)
- Quoting rules (escape characters for embedded delimiters)
- Header row presence
- Character encoding (UTF-8 is the modern default)

## When To Use

- Exchange of tabular data between tools
- Storing time-series records per airport / per device
- Input to Pandas DataFrames via `read_csv()`
- Long-term archival in a portable, human-readable format

## Risks & Pitfalls

- Embedded commas in numeric strings break naive parsing — pre-clean before saving
- Inconsistent quoting between writers
- Date and number locale ambiguity (DD/MM vs. MM/DD, `.` vs. `,` decimal)
- No native column-type metadata; downstream code must coerce types

## Related Concepts

- [[concepts/data-cleaning]]
- [[concepts/dataframe]]
- [[concepts/regular-expression]]
- [[entities/pandas]]

## Sources

- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
