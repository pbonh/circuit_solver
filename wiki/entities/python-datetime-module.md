---
title: Python `datetime` Module
type: entity
id: entities/python-datetime-module
tags:
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/05-chapter-1-getting-familiar-with-python.txt
---

## Overview

`datetime` is a standard-library Python module for representing dates, times, durations, and combinations thereof. The book uses it to define `date`, `time`, `datetime`, and `timedelta` objects.

## Characteristics

- `date(year, month, day)` requires all three arguments
- `time(hour, minute, second, microsecond)` arguments are optional
- `datetime(...)` combines date and time
- `timedelta` represents a duration; supports addition with `date` and `datetime` but not `time`

## Common Strategies

- Build date arithmetic via `timedelta`
- Parse/format strings with `strptime`/`strftime` (referenced in further reading)
- Combine with Pandas time-series for richer indexing

## Related Entities

- [[entities/pandas]]

## Sources

- [[summaries/python-data-analysts-toolkit-05-chapter-1-getting-familiar-with-python]]
