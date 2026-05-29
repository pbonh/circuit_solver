---
title: ATADS Dataset
type: entity
id: entities/atads-dataset
tags:
- aviation
- dataset
- time-series
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/04-introduction.txt
---

## Overview

The Air Traffic Activity System (ATADS) dataset is published by the U.S. Federal Aviation Administration (FAA) and records daily flight-operations counts at more than 500 U.S. airports. It is the central dataset used throughout *Prototyping Python Dashboards for Scientists and Engineers* and motivates the book's end-to-end pipeline of screen scraping, format conversion, dashboard construction, and deployment.

## Characteristics

- More than 500 reporting airports with ~20 years of history.
- Per-airport daily breakdown by operation category: local vs. itinerant, civilian vs. military, air carrier, air taxi, general aviation, and IFR vs. VFR.
- Distributed as HTML, Word, or Excel from a multi-step web form (no direct API).
- Roughly 170 MB per year as downloaded; downsizes to ~300 MB total in CSV form for all years.
- Updated monthly with new operations data.
- Each three-letter airport code (e.g., PHX, JFK, LAX, JNU, GCN, ANC, DEN, OSH, FLG, FAI) maps to a separate per-airport CSV in the book's pipeline.

## Common Strategies

- Use Selenium + ChromeDriver to navigate the FAA web form and trigger downloads.
- Convert downloaded Excel-HTML to CSV with `xls2csv.py`, stripping markup, removing thousands-separator commas, and trimming the trailing space from three-letter codes.
- Split annual CSVs into per-airport CSVs (`split_by_apt.py`) so dashboards load only a small slice per session.
- Refresh monthly via cron or systemd timers; rebuild rather than incrementally append for clarity.

## Related Entities

- [[entities/bts-t100-dataset]]
- [[entities/selenium]]
- [[entities/chromedriver]]
- [[entities/pandas]]
- [[entities/avopsinsight]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
